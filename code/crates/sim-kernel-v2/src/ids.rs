use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

string_id!(ActorId);
string_id!(CommodityId);
string_id!(EventId);
string_id!(InventoryAccountId);
string_id!(MarketId);
string_id!(MoneyAccountId);
string_id!(PlaceId);
string_id!(RouteEdgeId);
string_id!(ShipmentId);
string_id!(TransactionId);
string_id!(UnitId);

#[cfg(test)]
mod tests {
    use super::{ActorId, EventId, RouteEdgeId, TransactionId, UnitId};

    #[test]
    fn stable_id_keeps_exact_external_value() {
        let id = ActorId::new("merchant.otc.001");
        assert_eq!(id.as_str(), "merchant.otc.001");
        assert_eq!(id.to_string(), "merchant.otc.001");
    }

    #[test]
    fn transaction_ids_are_distinct_typed_values() {
        let id = TransactionId::new("tx.seed.001");
        assert_eq!(id.as_str(), "tx.seed.001");
    }

    #[test]
    fn event_ids_are_stable_external_values() {
        let id = EventId::new("event.production.batch.001");
        assert_eq!(id.as_str(), "event.production.batch.001");
    }

    #[test]
    fn scenario_registry_ids_remain_exact() {
        assert_eq!(UnitId::new("unit.kg").as_str(), "unit.kg");
        assert_eq!(
            RouteEdgeId::new("route.waterdeep-neverwinter").as_str(),
            "route.waterdeep-neverwinter"
        );
    }
}
