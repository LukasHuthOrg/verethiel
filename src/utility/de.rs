use std::collections::HashMap;

use serde::{de::Visitor, Deserialize};

use crate::utility::Translation;

impl<'de> Deserialize<'de> for Translation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(TranslationVisitor)
    }
}

#[allow(unused)]
struct TranslationVisitor;

impl<'de> Visitor<'de> for TranslationVisitor {
    type Value = Translation;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "string or map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Translation::Value(v.to_owned(), false))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut content = HashMap::new();
        let mut order = Vec::new();

        while let Some((key, value)) = map.next_entry::<String, Translation>()? {
            order.push(key.clone());
            content.insert(key, value);
        }
        Ok(Translation::Map { content, order })
    }
}
