use std::collections::{HashMap, VecDeque, vec_deque};

use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub(crate) enum Translation {
    Value(String),
    Map {
        content: HashMap<String, Translation>,
        order: Vec<String>,
    },
}

impl Translation {
    pub fn containsKey(&self, key: &[&String]) -> bool {
        match self {
            Self::Value(_) => key.is_empty(),
            Self::Map { content, .. } => {
                let Some(&first) = key.first() else {
                    return false;
                };
                let Some(entry) = content.get(first) else {
                    return false;
                };
                entry.containsKey(&key[1..])
            }
        }
    }
    pub fn getKeys(&self) -> Vec<VecDeque<&String>> {
        match self {
            Self::Value(_) => vec![vec![].into()],
            Self::Map { content, .. } => content
                .iter()
                .flat_map(move |(key, value)| {
                    value
                        .getKeys()
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

    assert!(translation.containsKey(&[&"a".to_string(), &"a".to_string()]));
    assert!(translation.containsKey(&[&"a".to_string(), &"b".to_string()]));
    assert!(translation.containsKey(&[&"b".to_string()]));
    assert!(translation.containsKey(&[&"c".to_string()]));

    let translation_keys: Vec<Vec<&String>> = translation
        .getKeys()
        .into_iter()
        .map(VecDeque::into_iter)
        .map(vec_deque::IntoIter::<&String>::collect)
        .collect();

    assert_eq!(translation_keys.len(), 4);
    for key in translation_keys.iter().map(Vec::as_slice) {
        assert!(translation.containsKey(key), "{key:?}");
    }
}
