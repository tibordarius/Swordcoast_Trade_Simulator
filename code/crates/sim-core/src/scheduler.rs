use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScheduledEvent {
    pub due_tick: u64,
    pub sequence: u64,
    pub kind: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Scheduler {
    queue: BinaryHeap<Reverse<ScheduledEvent>>,
}

impl PartialEq for Scheduler {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_events() == other.sorted_events()
    }
}

impl Eq for Scheduler {}

impl Scheduler {
    pub fn push(&mut self, event: ScheduledEvent) {
        self.queue.push(Reverse(event));
    }

    pub fn pop_due(&mut self, current_tick: u64) -> Option<ScheduledEvent> {
        match self.queue.peek() {
            Some(Reverse(event)) if event.due_tick <= current_tick => {
                self.queue.pop().map(|Reverse(event)| event)
            }
            _ => None,
        }
    }

    pub fn sorted_events(&self) -> Vec<ScheduledEvent> {
        let mut events: Vec<_> = self.queue.iter().map(|Reverse(event)| *event).collect();
        events.sort();
        events
    }
}
