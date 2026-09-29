#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryLedger {
    pub on_hand: i64,
    pub reserved: i64,
    pub target_reserve: i64,
    pub unmet_demand: i64,
    pub produced: i64,
    pub consumed: i64,
    pub spoiled: i64,
}

impl InventoryLedger {
    pub fn new(on_hand: i64, reserved: i64, target_reserve: i64) -> Self {
        assert!(on_hand >= 0);
        assert!(reserved >= 0 && reserved <= on_hand);
        assert!(target_reserve >= 0);
        Self {
            on_hand,
            reserved,
            target_reserve,
            unmet_demand: 0,
            produced: 0,
            consumed: 0,
            spoiled: 0,
        }
    }

    pub fn available(&self) -> i64 {
        self.on_hand - self.reserved
    }

    pub fn reserve(&mut self, amount: i64) -> bool {
        if amount <= 0 || amount > self.available() {
            return false;
        }
        self.reserved += amount;
        true
    }

    pub fn depart_reserved(&mut self, amount: i64) -> bool {
        if amount <= 0 || amount > self.reserved {
            return false;
        }
        self.reserved -= amount;
        self.on_hand -= amount;
        true
    }

    pub fn receive(&mut self, amount: i64) -> bool {
        if amount <= 0 {
            return false;
        }
        self.on_hand = self.on_hand.checked_add(amount).expect("inventory overflow");
        true
    }

    pub fn add_production(&mut self, amount: i64) {
        assert!(amount >= 0);
        self.on_hand = self.on_hand.checked_add(amount).expect("inventory overflow");
        self.produced = self.produced.checked_add(amount).expect("production counter overflow");
    }

    pub fn consume(&mut self, requested: i64) -> i64 {
        assert!(requested >= 0);
        let fulfilled = requested.min(self.available().max(0));
        self.on_hand -= fulfilled;
        self.consumed += fulfilled;
        self.unmet_demand += requested - fulfilled;
        fulfilled
    }

    pub fn apply_spoilage_ppm(&mut self, ppm: i64) -> i64 {
        const ONE_MILLION: i64 = 1_000_000;
        assert!((0..=ONE_MILLION).contains(&ppm));
        let loss = self.on_hand.saturating_mul(ppm) / ONE_MILLION;
        self.on_hand -= loss;
        self.spoiled += loss;
        self.reserved = self.reserved.min(self.on_hand);
        loss
    }

    pub fn conservation_total(&self) -> i64 {
        self.on_hand + self.consumed + self.spoiled
    }
}
