#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta {
    pub priority: u16,
    pub entity_id: u64,
    pub kind: u16,
    pub sequence: u64,
    pub amount: i64,
}

impl Delta {
    pub fn sort_key(&self) -> (u16, u64, u16, u64) {
        (self.priority, self.entity_id, self.kind, self.sequence)
    }
}

pub fn stable_sort_deltas(deltas: &mut [Delta]) {
    deltas.sort_by_key(Delta::sort_key);
}
