#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactDailyRate {
    amount_per_day: i64,
    hours_elapsed: i64,
    emitted: i64,
}

impl ExactDailyRate {
    pub fn new(amount_per_day: i64) -> Self {
        assert!(amount_per_day >= 0);
        Self {
            amount_per_day,
            hours_elapsed: 0,
            emitted: 0,
        }
    }

    pub fn next_hour(&mut self) -> i64 {
        self.hours_elapsed += 1;
        let target = self.amount_per_day.saturating_mul(self.hours_elapsed) / 24;
        let due = target - self.emitted;
        self.emitted = target;
        due
    }
}
