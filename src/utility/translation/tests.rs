use std::collections::{VecDeque, vec_deque};

use crate::utility::translation::Translation;

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
#[should_panic(expected = "trailing comma")]
fn test_panic_on_invalid_comma() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b",}, "b": "b"}"#;
    let _: Translation = serde_json::from_str(INPUT).unwrap();
}
#[test]
#[should_panic(expected = "duplicate key")]
fn test_panic_on_duplicate_key() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b", "a": "?"}, "b": "b"}"#;
    let _: Translation = serde_json::from_str(INPUT).unwrap();
}

