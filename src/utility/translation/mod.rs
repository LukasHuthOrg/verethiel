use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Translation {
    Value(String, bool),
    Map {
        content: HashMap<String, Translation>,
        order: Vec<String>,
    },
}
impl Default for Translation {
    fn default() -> Self {
        panic!(
            "This should never be called, since Vec::default() does Vec::new(), but needs T::default"
        )
    }
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
        let current_order_keys = current_order.clone().into_iter().collect::<HashSet<_>>();
        let intended_order_keys = intended_order.clone().into_iter().collect::<HashSet<_>>();
        let in_both = intended_order
            .iter()
            .filter(|&key| current_order_keys.contains(key))
            .cloned();
        let only_current = current_order
            .iter()
            .filter(|&key| !intended_order_keys.contains(key))
            .cloned();
        *current_order = in_both.chain(only_current).collect();
        for (key, value) in content {
            if let Some(intended) = intended_content.get(key) {
                value.apply_translation_order(intended)?;
            }
        }
        Ok(())
    }
    pub fn visit_translation(&mut self, other: &Translation) -> Result<(), String> {
        for key in other.get_keys() {
            let key = key.into_iter().collect::<Vec<_>>();
            let key = key.as_slice();
            if self.visit_key(key).is_err() {
                let key = key.to_string();
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
                let key = key.to_string();
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
                    let position = Self::get_position(order, key);
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
    fn get_position(order: &Vec<String>, key: &String) -> usize {
        order
            .iter()
            .enumerate()
            .filter(|&(_, e)| e == key)
            .map(|(i, _)| i)
            .next()
            .expect("There should always be an entry in ordered")
    }
    pub fn get_keys(&self) -> Vec<VecDeque<&String>> {
        match self {
            Self::Value(_, _) => vec![vec![].into()],
            Self::Map { content, .. } => content.iter().flat_map(Self::get_keys_int).collect(),
        }
    }
    fn get_keys_int<'a>(input: (&'a String, &'a Translation)) -> Vec<VecDeque<&'a String>> {
        let (key, value) = input;
        value
            .get_keys()
            .into_iter()
            .map(|mut arr| {
                arr.push_front(key);
                arr
            })
            .collect::<Vec<_>>()
    }
}
impl ToString for Translation {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .expect("A Translation should always be able to be converted to json")
    }
}
pub trait IntoKey<'a> {
    fn to_key(self) -> Vec<&'a String>;
}
impl<'a> IntoKey<'a> for VecDeque<&'a String> {
    fn to_key(self) -> Vec<&'a String> {
        self.into_iter().collect::<Vec<_>>()
    }
}
pub trait KeyToString {
    fn to_string(self) -> String;
}
impl KeyToString for &[&String] {
    fn to_string(self) -> String {
        self.into_iter()
            .map(|&s| s.clone())
            .collect::<Vec<String>>()
            .join(".")
    }
}
impl KeyToString for &[(&String, usize)] {
    fn to_string(self) -> String {
        self.into_iter()
            .map(|&(s, _)| s.clone())
            .collect::<Vec<String>>()
            .join(".")
    }
}
impl KeyToString for VecDeque<&String> {
    fn to_string(self) -> String {
        self.into_iter().cloned().collect::<Vec<_>>().join(".")
    }
}
pub trait ToKeyArray<'a> {
    fn keys(self) -> Vec<Vec<&'a String>>;
}
impl<'a, A, I> ToKeyArray<'a> for A
where
    A: IntoIterator<Item = I>,
    I: IntoKey<'a>,
{
    fn keys(self) -> Vec<Vec<&'a String>> {
        self.into_iter().map(I::to_key).collect()
    }
}
