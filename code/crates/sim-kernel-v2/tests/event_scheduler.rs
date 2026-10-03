use sim_kernel_v2::{
    decode_snapshot, encode_snapshot, replay, state_hash, Command, CommandEnvelope, EventDomain,
    EventId, SimTick, WorldReducer, WorldState,
};

fn schedule(sequence: u64, id: &str, tick: u64, domain: EventDomain) -> CommandEnvelope {
    CommandEnvelope::new(
        sequence,
        Command::ScheduleEvent {
            event_id: EventId::new(id),
            at_tick: SimTick::new(tick),
            domain,
        },
    )
}

#[test]
fn same_tick_events_fire_by_domain_then_insertion_sequence() {
    let mut state = WorldState::new(1);

    WorldReducer::apply(
        &mut state,
        &schedule(1, "logistics.first", 10, EventDomain::Logistics),
    )
    .unwrap();
    WorldReducer::apply(
        &mut state,
        &schedule(2, "production", 10, EventDomain::Production),
    )
    .unwrap();
    WorldReducer::apply(
        &mut state,
        &schedule(3, "logistics.second", 10, EventDomain::Logistics),
    )
    .unwrap();

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            4,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    )
    .unwrap();

    let fired: Vec<&str> = state
        .scheduler()
        .fired_events()
        .iter()
        .map(|event| event.event_id().as_str())
        .collect();

    assert_eq!(
        fired,
        vec!["production", "logistics.first", "logistics.second"]
    );
}

#[test]
fn cancelled_event_never_fires() {
    let mut state = WorldState::new(1);
    let event_id = EventId::new("cancel.me");

    WorldReducer::apply(
        &mut state,
        &schedule(1, "cancel.me", 10, EventDomain::Information),
    )
    .unwrap();

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            2,
            Command::CancelEvent {
                event_id: event_id.clone(),
            },
        ),
    )
    .unwrap();

    assert!(!state.scheduler().is_active(&event_id));

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            3,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    )
    .unwrap();

    assert!(state.scheduler().fired_events().is_empty());
    assert_eq!(state.scheduler().queued_entry_count(), 0);
}

#[test]
fn rescheduling_same_id_invalidates_old_generation() {
    let mut state = WorldState::new(1);
    let event_id = EventId::new("shipment.arrival");

    WorldReducer::apply(
        &mut state,
        &schedule(1, "shipment.arrival", 20, EventDomain::Logistics),
    )
    .unwrap();
    WorldReducer::apply(
        &mut state,
        &schedule(2, "shipment.arrival", 10, EventDomain::Logistics),
    )
    .unwrap();

    assert_eq!(state.scheduler().active_event_count(), 1);
    assert_eq!(state.scheduler().queued_entry_count(), 2);

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            3,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    )
    .unwrap();

    assert_eq!(state.scheduler().fired_events().len(), 1);
    assert_eq!(state.scheduler().fired_events()[0].event_id(), &event_id);
    assert_eq!(state.scheduler().fired_events()[0].generation(), 2);
    assert_eq!(state.scheduler().queued_entry_count(), 1);

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            4,
            Command::AdvanceTo {
                tick: SimTick::new(20),
            },
        ),
    )
    .unwrap();

    assert_eq!(state.scheduler().fired_events().len(), 1);
    assert_eq!(state.scheduler().queued_entry_count(), 0);
}

#[test]
fn advance_drains_every_valid_event_through_target_tick() {
    let mut state = WorldState::new(1);

    for command in [
        schedule(1, "at.5", 5, EventDomain::System),
        schedule(2, "at.7", 7, EventDomain::Production),
        schedule(3, "at.12", 12, EventDomain::Shock),
    ] {
        WorldReducer::apply(&mut state, &command).unwrap();
    }

    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            4,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    )
    .unwrap();

    let fired: Vec<&str> = state
        .scheduler()
        .fired_events()
        .iter()
        .map(|event| event.event_id().as_str())
        .collect();

    assert_eq!(fired, vec!["at.5", "at.7"]);
    assert_eq!(state.scheduler().active_event_count(), 1);
    assert_eq!(state.tick(), SimTick::new(10));
}

#[test]
fn snapshot_roundtrip_preserves_pending_and_fired_scheduler_state() {
    let mut state = WorldState::new(77);

    WorldReducer::apply(
        &mut state,
        &schedule(1, "fires.first", 5, EventDomain::Production),
    )
    .unwrap();
    WorldReducer::apply(
        &mut state,
        &schedule(2, "stays.pending", 15, EventDomain::Logistics),
    )
    .unwrap();
    WorldReducer::apply(
        &mut state,
        &CommandEnvelope::new(
            3,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    )
    .unwrap();

    let before = state_hash(&state).unwrap();
    let encoded = encode_snapshot(&state).unwrap();
    let decoded = decode_snapshot(&encoded).unwrap();

    assert_eq!(before, state_hash(&decoded).unwrap());
    assert_eq!(decoded.scheduler().fired_events().len(), 1);
    assert_eq!(decoded.scheduler().active_event_count(), 1);
    assert_eq!(state, decoded);
}

#[test]
fn replay_reproduces_event_order_and_state_hash() {
    let commands = vec![
        schedule(1, "logistics", 10, EventDomain::Logistics),
        schedule(2, "production", 10, EventDomain::Production),
        CommandEnvelope::new(
            3,
            Command::AdvanceTo {
                tick: SimTick::new(10),
            },
        ),
    ];

    let initial = WorldState::new(123);
    let a = replay(&initial, &commands).unwrap();
    let b = replay(&initial, &commands).unwrap();

    assert_eq!(state_hash(&a).unwrap(), state_hash(&b).unwrap());

    let a_ids: Vec<&str> = a
        .scheduler()
        .fired_events()
        .iter()
        .map(|event| event.event_id().as_str())
        .collect();
    let b_ids: Vec<&str> = b
        .scheduler()
        .fired_events()
        .iter()
        .map(|event| event.event_id().as_str())
        .collect();

    assert_eq!(a_ids, b_ids);
    assert_eq!(a_ids, vec!["production", "logistics"]);
}
