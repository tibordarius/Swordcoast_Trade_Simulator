use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{EventId, SimTick};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum EventDomain {
    System,
    Production,
    Logistics,
    Information,
    Shock,
}

impl EventDomain {
    #[must_use]
    pub const fn priority(self) -> u8 {
        match self {
            Self::System => 0,
            Self::Production => 10,
            Self::Logistics => 20,
            Self::Information => 30,
            Self::Shock => 40,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduledEvent {
    event_id: EventId,
    at_tick: SimTick,
    domain: EventDomain,
    sequence: u64,
    generation: u64,
}

impl ScheduledEvent {
    #[must_use]
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    #[must_use]
    pub const fn at_tick(&self) -> SimTick {
        self.at_tick
    }

    #[must_use]
    pub const fn domain(&self) -> EventDomain {
        self.domain
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FiredEvent {
    event_id: EventId,
    at_tick: SimTick,
    domain: EventDomain,
    sequence: u64,
    generation: u64,
}

impl FiredEvent {
    #[must_use]
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    #[must_use]
    pub const fn at_tick(&self) -> SimTick {
        self.at_tick
    }

    #[must_use]
    pub const fn domain(&self) -> EventDomain {
        self.domain
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

impl From<ScheduledEvent> for FiredEvent {
    fn from(event: ScheduledEvent) -> Self {
        Self {
            event_id: event.event_id,
            at_tick: event.at_tick,
            domain: event.domain,
            sequence: event.sequence,
            generation: event.generation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
struct EventOrderKey {
    at_tick: SimTick,
    domain_priority: u8,
    sequence: u64,
}

impl EventOrderKey {
    fn for_event(event: &ScheduledEvent) -> Self {
        Self {
            at_tick: event.at_tick,
            domain_priority: event.domain.priority(),
            sequence: event.sequence,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventSchedulerState {
    queue: BTreeMap<EventOrderKey, ScheduledEvent>,
    versions: BTreeMap<EventId, u64>,
    active: BTreeMap<EventId, u64>,
    next_sequence: u64,
    fired_events: Vec<FiredEvent>,
}

impl EventSchedulerState {
    #[must_use]
    pub fn fired_events(&self) -> &[FiredEvent] {
        &self.fired_events
    }

    #[must_use]
    pub fn active_event_count(&self) -> usize {
        self.active.len()
    }

    #[must_use]
    pub fn queued_entry_count(&self) -> usize {
        self.queue.len()
    }

    #[must_use]
    pub fn is_active(&self, event_id: &EventId) -> bool {
        self.active.contains_key(event_id)
    }

    pub(crate) fn next_sequence(&self) -> Option<u64> {
        self.next_sequence.checked_add(1)
    }

    pub(crate) fn next_generation(&self, event_id: &EventId) -> Option<u64> {
        self.versions
            .get(event_id)
            .copied()
            .unwrap_or(0)
            .checked_add(1)
    }

    pub(crate) fn commit_schedule(
        &mut self,
        event_id: EventId,
        at_tick: SimTick,
        domain: EventDomain,
        sequence: u64,
        generation: u64,
    ) {
        self.next_sequence = sequence;
        self.versions.insert(event_id.clone(), generation);
        self.active.insert(event_id.clone(), generation);

        let event = ScheduledEvent {
            event_id,
            at_tick,
            domain,
            sequence,
            generation,
        };

        self.queue.insert(EventOrderKey::for_event(&event), event);
    }

    pub(crate) fn commit_cancel(&mut self, event_id: &EventId, generation: u64) {
        self.versions.insert(event_id.clone(), generation);
        self.active.remove(event_id);
    }

    pub(crate) fn drain_due(&mut self, through: SimTick) {
        let keys: Vec<EventOrderKey> = self
            .queue
            .keys()
            .take_while(|key| key.at_tick <= through)
            .copied()
            .collect();

        for key in keys {
            let event = self
                .queue
                .remove(&key)
                .expect("queued event key disappeared during deterministic drain");

            match self.active.get(event.event_id()) {
                Some(active_generation) if *active_generation == event.generation() => {
                    self.active.remove(event.event_id());
                    self.fired_events.push(event.into());
                }
                _ => {
                    // Old generations remain queued until their scheduled tick.
                    // When encountered, they are discarded without firing.
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EventDomain, EventSchedulerState};
    use crate::{EventId, SimTick};

    #[test]
    fn domain_priority_is_explicit_and_stable() {
        assert!(EventDomain::Production.priority() < EventDomain::Logistics.priority());
        assert!(EventDomain::Logistics.priority() < EventDomain::Information.priority());
    }

    #[test]
    fn deterministic_drain_orders_domain_then_sequence() {
        let mut scheduler = EventSchedulerState::default();

        let logistics_id = EventId::new("logistics");
        let logistics_sequence = scheduler.next_sequence().unwrap();
        let logistics_generation = scheduler.next_generation(&logistics_id).unwrap();
        scheduler.commit_schedule(
            logistics_id,
            SimTick::new(10),
            EventDomain::Logistics,
            logistics_sequence,
            logistics_generation,
        );

        let production_id = EventId::new("production");
        let production_sequence = scheduler.next_sequence().unwrap();
        let production_generation = scheduler.next_generation(&production_id).unwrap();
        scheduler.commit_schedule(
            production_id,
            SimTick::new(10),
            EventDomain::Production,
            production_sequence,
            production_generation,
        );

        scheduler.drain_due(SimTick::new(10));

        let ids: Vec<&str> = scheduler
            .fired_events()
            .iter()
            .map(|event| event.event_id().as_str())
            .collect();

        assert_eq!(ids, vec!["production", "logistics"]);
    }
}
