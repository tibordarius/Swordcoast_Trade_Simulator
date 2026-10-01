use serde::{Deserialize, Serialize};

/// Integral copper-piece amount.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MoneyCp(pub i128);

impl MoneyCp {
    pub const ZERO: Self = Self(0);
    #[must_use] pub const fn new(value: i128) -> Self { Self(value) }
    #[must_use] pub const fn get(self) -> i128 { self.0 }
    #[must_use] pub fn checked_add(self, rhs: Self) -> Option<Self> { self.0.checked_add(rhs.0).map(Self) }
    #[must_use] pub fn checked_sub(self, rhs: Self) -> Option<Self> { self.0.checked_sub(rhs.0).map(Self) }
}

/// Integral base-unit commodity quantity.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Quantity(pub i64);

impl Quantity {
    pub const ZERO: Self = Self(0);
    #[must_use] pub const fn new(value: i64) -> Self { Self(value) }
    #[must_use] pub const fn get(self) -> i64 { self.0 }
}

/// Fixed-point copper pieces per commodity base unit.
/// scaled_value divided by scale equals copper pieces per base unit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UnitPrice {
    scaled_value: i128,
    scale: u32,
}

impl UnitPrice {
    pub const DEFAULT_SCALE: u32 = 1_000;

    pub fn new(scaled_value: i128, scale: u32) -> Result<Self, UnitPriceError> {
        if scale == 0 { return Err(UnitPriceError::ZeroScale); }
        Ok(Self { scaled_value, scale })
    }

    #[must_use] pub const fn from_milli_cp(scaled_value: i128) -> Self {
        Self { scaled_value, scale: Self::DEFAULT_SCALE }
    }

    #[must_use] pub const fn scaled_value(self) -> i128 { self.scaled_value }
    #[must_use] pub const fn scale(self) -> u32 { self.scale }

    pub fn total_for(self, quantity: Quantity) -> Result<MoneyCp, UnitPriceError> {
        let numerator = self.scaled_value.checked_mul(i128::from(quantity.0)).ok_or(UnitPriceError::Overflow)?;
        Ok(MoneyCp(numerator / i128::from(self.scale)))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnitPriceError { ZeroScale, Overflow }

#[cfg(test)]
mod tests {
    use super::{MoneyCp, Quantity, UnitPrice, UnitPriceError};

    #[test]
    fn supports_sub_copper_unit_prices_without_floating_point() {
        let price = UnitPrice::from_milli_cp(125);
        assert_eq!(price.total_for(Quantity::new(8)).unwrap(), MoneyCp::new(1));
    }

    #[test]
    fn rejects_zero_scale() {
        assert_eq!(UnitPrice::new(1, 0), Err(UnitPriceError::ZeroScale));
    }

    #[test]
    fn money_arithmetic_is_checked() {
        assert_eq!(MoneyCp::new(10).checked_sub(MoneyCp::new(3)), Some(MoneyCp::new(7)));
        assert_eq!(MoneyCp::new(i128::MAX).checked_add(MoneyCp::new(1)), None);
    }
}