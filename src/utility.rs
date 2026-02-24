use std::collections::{vec_deque, HashMap, VecDeque};

use serde::Serialize;

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
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

fn to_slice(a: Vec<VecDeque<&String>>) -> Vec<Vec<&String>> {
    a.into_iter()
        .map(VecDeque::into_iter)
        .map(vec_deque::IntoIter::<&String>::collect)
        .collect()
}

mod de;

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
                        ("a".to_string(), Translation::Value("a".to_string())),
                        ("b".to_string(), Translation::Value("b".to_string())),
                    ]
                    .into_iter()
                    .collect(),
                    order: vec!["a".to_string(), "b".to_string()],
                },
            ),
            ("b".to_string(), Translation::Value("b".to_string())),
            ("c".to_string(), Translation::Value("c".to_string())),
        ]
        .into_iter()
        .collect(),
        order: vec!["c".to_string(), "a".to_string(), "b".to_string()],
    };

    assert_eq!(translation, expected);

    assert!(translation.contains_key(&[&"a".to_string(), &"a".to_string()]));
    assert!(translation.contains_key(&[&"a".to_string(), &"b".to_string()]));
    assert!(translation.contains_key(&[&"b".to_string()]));
    assert!(translation.contains_key(&[&"c".to_string()]));

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
