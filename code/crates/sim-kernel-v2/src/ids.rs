use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
            #[must_use] pub fn as_str(&self) -> &str { &self.0 }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
        }
    };
}

string_id!(ActorId);
string_id!(CommodityId);
string_id!(MarketId);
string_id!(ShipmentId);

#[cfg(test)]
mod tests {
    use super::ActorId;

    #[test]
    fn stable_id_keeps_exact_external_value() {
        let id = ActorId::new("merchant.otc.001");
        assert_eq!(id.as_str(), "merchant.otc.001");
        assert_eq!(id.to_string(), "merchant.otc.001");
    }
}