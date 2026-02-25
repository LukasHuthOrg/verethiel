use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Translation {
    Value(String, bool),
    Map {
        content: HashMap<String, Translation>,
        order: Vec<String>,
    },
}

mod de;
mod ser;

#[cfg(test)]
mod tests;

impl Translation {
    #[allow(unused)]
    pub fn contains_key(&self, key: &[&String]) -> bool {
        match self {
            Self::Value(_, _) => key.is_empty(),
            Self::Map { content, .. } => {
                let Some(&first) = key.first() else {
                    return false;
                };
                let Some(entry) = content.get(first) else {
                    return false;
                };
                entry.contains_key(&key[1..])
            }
        }
    }
    pub fn apply_translation_order(&mut self, other: &Translation) -> Result<(), String> {
        if matches!(
            (&self, other),
            (Translation::Value(_, _), Translation::Value(_, _))
        ) {
            return Ok(());
        }
        let Translation::Map {
            order: current_order,
            content,
        } = self
        else {
            return Err("self and other diverged, self being value, other map".to_string());
        };
        let Translation::Map {
            order: intended_order,
            content: intended_content,
        } = other
        else {
            return Err("self and other diverged, self being map, other value".to_string());
        };
        *current_order = intended_order.clone();
        for (key, value) in content {
            let intended = intended_content
                .get(key)
                .ok_or(format!("'{key}' not found in other but self"))?;
            value.apply_translation_order(intended)?;
        }
        Ok(())
    }
    pub fn visit_translation(&mut self, other: &Translation) -> Result<(), String> {
        for key in other.get_keys() {
            let key = key.into_iter().collect::<Vec<_>>();
            let key = key.as_slice();
            if self.visit_key(key).is_err() {
                let key = key
                    .into_iter()
                    .map(|&s| s.clone())
                    .collect::<Vec<String>>()
                    .join(".");
                return Err(key);
            }
        }
        Ok(())
    }
    pub fn visit_key(&mut self, key: &[&String]) -> Result<(), ()> {
        match self {
            Self::Value(_, visited) => {
                *visited = true;
                Ok(())
            }
            Self::Map { content, .. } => {
                let Some(&first) = key.first() else {
                    return Err(());
                };
                let Some(entry) = content.get_mut(first) else {
                    return Err(());
                };
                entry.visit_key(&key[1..])
            }
        }
    }
    pub fn everything_visited(&self) -> bool {
        match self {
            Self::Value(_, visited) => *visited,
            Self::Map { content, .. } => content.values().all(Self::everything_visited),
        }
    }
    pub fn visit_ordered_translation<const CANCEL_ON_MISS: bool>(
        &mut self,
        other: &Translation,
    ) -> Result<(), String> {
        for key in other.get_ordered_keys() {
            let key = key.into_iter().collect::<Vec<_>>();
            let key = key.as_slice();
            if self.visit_ordered_key::<CANCEL_ON_MISS>(key).is_err() {
                let key = key
                    .into_iter()
                    .map(|&(s, _)| s.clone())
                    .collect::<Vec<String>>()
                    .join(".");
                return Err(key);
            }
        }
        Ok(())
    }
    pub fn visit_ordered_key<const CANCEL_ON_MISS: bool>(
        &mut self,
        key: &[(&String, usize)],
    ) -> Result<(), ()> {
        match self {
            Self::Value(_, visited) => {
                *visited = true;
                Ok(())
            }
            Self::Map { content, order } => {
                let Some(&(first, position)) = key.first() else {
                    return Err(());
                };
                let is_at_expected_position = order.get(position).is_some_and(|v| v == first);
                if !is_at_expected_position {
                    return if CANCEL_ON_MISS { Err(()) } else { Ok(()) };
                }
                let Some(entry) = content.get_mut(first) else {
                    return Err(());
                };
                entry.visit_ordered_key::<CANCEL_ON_MISS>(&key[1..])
            }
        }
    }
    pub fn get_ordered_keys(&self) -> Vec<VecDeque<(&String, usize)>> {
        match self {
            Self::Value(_, _) => vec![vec![].into()],
            Self::Map { content, order } => content
                .iter()
                .flat_map(move |(key, value)| {
                    let position = order
                        .iter()
                        .enumerate()
                        .filter(|&(_, e)| e == key)
                        .map(|(i, _)| i)
                        .next()
                        .expect("There should always be an entry in ordered");
                    value
                        .get_ordered_keys()
                        .into_iter()
                        .map(|mut arr| {
                            arr.push_front((key, position));
                            arr
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }
    pub fn get_keys(&self) -> Vec<VecDeque<&String>> {
        match self {
            Self::Value(_, _) => vec![vec![].into()],
            Self::Map { content, .. } => content
                .iter()
                .flat_map(move |(key, value)| {
                    value
                        .get_keys()
                        .into_iter()
                        .map(|mut arr| {
                            arr.push_front(key);
                            arr
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        }
    }
}
impl ToString for Translation {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .expect("A Translation should always be able to be converted to json")
    }
}
