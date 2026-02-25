use std::collections::{HashMap, VecDeque, vec_deque};

use serde::Serialize;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Translation {
    Value(String, bool),
    Map {
        content: HashMap<String, Translation>,
        order: Vec<String>,
    },
}

impl Translation {
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

mod de;
mod ser;

#[test]
fn test_deserialization() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b"}"#;
    let translation: Translation = serde_json::from_str(INPUT).unwrap();
    let expected = Translation::Map {
        content: vec![
            (
                "a".to_string(),
                Translation::Map {
                    content: vec![
                        ("a".to_string(), Translation::Value("a".to_string(), false)),
                        ("b".to_string(), Translation::Value("b".to_string(), false)),
                    ]
                    .into_iter()
                    .collect(),
                    order: vec!["a".to_string(), "b".to_string()],
                },
            ),
            ("b".to_string(), Translation::Value("b".to_string(), false)),
            ("c".to_string(), Translation::Value("c".to_string(), false)),
        ]
        .into_iter()
        .collect(),
        order: vec!["c".to_string(), "a".to_string(), "b".to_string()],
    };

    assert_eq!(translation, expected);
}

#[test]
fn test_serialization() {
    let value = Translation::Map {
        content: vec![
            (
                "a".to_string(),
                Translation::Map {
                    content: vec![
                        ("a".to_string(), Translation::Value("a".to_string(), false)),
                        ("b".to_string(), Translation::Value("b".to_string(), false)),
                    ]
                    .into_iter()
                    .collect(),
                    order: vec!["a".to_string(), "b".to_string()],
                },
            ),
            ("b".to_string(), Translation::Value("b".to_string(), false)),
            ("c".to_string(), Translation::Value("c".to_string(), false)),
        ]
        .into_iter()
        .collect(),
        order: vec!["c".to_string(), "a".to_string(), "b".to_string()],
    };

    let translation = serde_json::to_string(&value).unwrap();
    const EXPECTED: &str = r#"{"c":"c","a":{"a":"a","b":"b"},"b":"b"}"#;
    assert_eq!(translation, EXPECTED);
}

#[test]
fn test_contains_key() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b"}"#;
    let translation: Translation = serde_json::from_str(INPUT).unwrap();

    assert!(translation.contains_key(&[&"a".to_string(), &"a".to_string()]));
    assert!(translation.contains_key(&[&"a".to_string(), &"b".to_string()]));
    assert!(translation.contains_key(&[&"b".to_string()]));
    assert!(translation.contains_key(&[&"c".to_string()]));
}
#[test]
fn test_get_keys() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b"}"#;
    let translation: Translation = serde_json::from_str(INPUT).unwrap();

    let translation_keys: Vec<Vec<&String>> = translation
        .get_keys()
        .into_iter()
        .map(VecDeque::into_iter)
        .map(vec_deque::IntoIter::<&String>::collect)
        .collect();

    assert_eq!(translation_keys.len(), 4);
    for key in translation_keys.iter().map(Vec::as_slice) {
        assert!(translation.contains_key(key), "{key:?}");
    }
}
#[test]
fn test_get_ordered_keys() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b"}"#;
    let translation: Translation = serde_json::from_str(INPUT).unwrap();

    let ordered_keys: Vec<Vec<(&String, usize)>> = translation
        .get_ordered_keys()
        .into_iter()
        .map(VecDeque::into_iter)
        .map(vec_deque::IntoIter::collect)
        .collect();
    assert!(ordered_keys.contains(&vec![(&"c".to_string(), 0)]));
    assert!(ordered_keys.contains(&vec![(&"a".to_string(), 1), (&"a".to_string(), 0)]));
    assert!(ordered_keys.contains(&vec![(&"a".to_string(), 1), (&"b".to_string(), 1)]));
    assert!(ordered_keys.contains(&vec![(&"b".to_string(), 2)]));
}
#[test]
#[should_panic]
fn test_panic_on_invalid_comma() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b",}, "b": "b"}"#;
    let _: Translation = serde_json::from_str(INPUT).unwrap();
}
#[test]
#[should_panic]
fn test_panic_on_duplicate_key() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b", "a": "?"}, "b": "b"}"#;
    let _: Translation = serde_json::from_str(INPUT).unwrap();
}
