use serde::{ser::SerializeMap, Serialize};

use serde::ser::Error as _;

use super::Translation;

impl Serialize for Translation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Value(value, _) => serializer.serialize_str(value),
            Self::Map { content, order } => {
                let mut map = serializer.serialize_map(Some(order.len()))?;
                for key in order {
                    let value = content.get(key).ok_or(S::Error::custom(
                        "Everything in order should be in content aswell",
                    ))?;
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}
