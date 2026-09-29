use std::collections::BTreeMap;

use crate::{
    derive_stream_seed, fnv1a64, stable_sort_deltas, Delta, ScheduledEvent, Scheduler,
    SimulationClock, SplitMix64, StableStateHash,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldState {
    seed: u64,
    clock: SimulationClock,
    streams: BTreeMap<String, SplitMix64>,
    scheduler: Scheduler,
    deltas: Vec<Delta>,
    next_sequence: u64,
    production_signal: i64,
}

impl WorldState {
    const PRODUCTION_EVENT: u32 = 1;
    const PRODUCTION_DELTA: u16 = 1;

    pub fn new(seed: u64) -> Self {
        let mut streams = BTreeMap::new();
        streams.insert(
            "events".to_string(),
            SplitMix64::new(derive_stream_seed(seed, "events")),
        );
        streams.insert(
            "production".to_string(),
            SplitMix64::new(derive_stream_seed(seed, "production")),
        );

        let mut world = Self {
            seed,
            clock: SimulationClock::five_minute(),
            streams,
            scheduler: Scheduler::default(),
            deltas: Vec::new(),
            next_sequence: 1,
            production_signal: 0,
        };
        world.schedule(12, Self::PRODUCTION_EVENT);
        world
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn tick(&self) -> u64 {
        self.clock.tick()
    }

    pub fn production_signal(&self) -> i64 {
        self.production_signal
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    pub fn draw_from_stream(&mut self, namespace: &str) -> u64 {
        self.streams
            .get_mut(namespace)
            .unwrap_or_else(|| panic!("unknown RNG stream: {namespace}"))
            .next_u64()
    }

    fn allocate_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .expect("sequence overflow");
        sequence
    }

    fn schedule(&mut self, due_tick: u64, kind: u32) {
        let event = ScheduledEvent {
            due_tick,
            sequence: self.allocate_sequence(),
            kind,
        };
        self.scheduler.push(event);
    }

    fn emit_delta(&mut self, priority: u16, entity_id: u64, kind: u16, amount: i64) {
        let delta = Delta {
            priority,
            entity_id,
            kind,
            sequence: self.allocate_sequence(),
            amount,
        };
        self.deltas.push(delta);
    }

    fn handle_event(&mut self, event: ScheduledEvent) {
        match event.kind {
            Self::PRODUCTION_EVENT => {
                let draw = self.draw_from_stream("production");
                let amount = (draw % 7) as i64 - 3;
                self.emit_delta(100, 1, Self::PRODUCTION_DELTA, amount);
                self.schedule(event.due_tick + 12, Self::PRODUCTION_EVENT);
            }
            other => panic!("unknown event kind: {other}"),
        }
    }

    fn commit_deltas(&mut self) {
        stable_sort_deltas(&mut self.deltas);
        for delta in self.deltas.drain(..) {
            match (delta.entity_id, delta.kind) {
                (1, Self::PRODUCTION_DELTA) => self.production_signal += delta.amount,
                _ => panic!("unknown delta: {delta:?}"),
            }
        }
    }

    pub fn advance_one(&mut self) {
        self.clock.advance_one();
        while let Some(event) = self.scheduler.pop_due(self.clock.tick()) {
            self.handle_event(event);
        }
        self.commit_deltas();
    }

    pub fn run_ticks(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.advance_one();
        }
    }
}

impl StableStateHash for WorldState {
    fn stable_state_hash(&self) -> u64 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.tick().to_le_bytes());
        bytes.extend_from_slice(&self.seed.to_le_bytes());
        bytes.extend_from_slice(&self.production_signal.to_le_bytes());
        bytes.extend_from_slice(&self.next_sequence.to_le_bytes());

        for (namespace, stream) in &self.streams {
            let encoded = namespace.as_bytes();
            let len = u16::try_from(encoded.len()).expect("RNG namespace too long");
            bytes.extend_from_slice(&len.to_le_bytes());
            bytes.extend_from_slice(encoded);
            bytes.extend_from_slice(&stream.state().to_le_bytes());
        }

        for event in self.scheduler.sorted_events() {
            bytes.extend_from_slice(&event.due_tick.to_le_bytes());
            bytes.extend_from_slice(&event.sequence.to_le_bytes());
            bytes.extend_from_slice(&event.kind.to_le_bytes());
        }

        fnv1a64(&bytes)
    }
}
