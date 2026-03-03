use std::collections::{vec_deque, VecDeque};

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
#[should_panic(expected = "string or map")]
fn test_deserialization_unknown_struct() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b", "d": ["a", "b"]}"#;
    let _: Translation = serde_json::from_str(INPUT).unwrap();
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
#[should_panic(expected = "Everything in order should be in content aswell")]
fn test_serialization_faulty_stucture() {
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
            ("c".to_string(), Translation::Value("c".to_string(), false)),
        ]
        .into_iter()
        .collect(),
        order: vec!["c".to_string(), "a".to_string(), "b".to_string()],
    };

    _ = serde_json::to_string(&value).unwrap();
}


#[test]
fn test_contains_key() {
    const INPUT: &str = r#"{"c": "c", "a": {"a": "a", "b": "b"}, "b": "b"}"#;
    let translation: Translation = serde_json::from_str(INPUT).unwrap();

    assert!(!translation.contains_key(&[&"a".to_string()]));
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

#[test]
fn test_order_applyance() {
    const INPUT_A: &str = r#"{"a":"","b":""}"#;
    const INPUT_B: &str = r#"{"b":"b","a":"a"}"#;
    const EXPECTED: &str = r#"{"a":"a","b":"b"}"#;
    let t1: Translation = serde_json::from_str(INPUT_A).unwrap();
    let mut t2: Translation = serde_json::from_str(INPUT_B).unwrap();
    t2.apply_translation_order(&t1).unwrap();
    let result = serde_json::to_string(&t2).unwrap();
    assert_eq!(result, EXPECTED);
}
#[test]
#[should_panic(expected = "self and other diverged, self being value, other map")]
fn test_order_applyance_divergence_other() {
    const INPUT_A: &str = r#"{"a":"","b":{"b":""}}"#;
    const INPUT_B: &str = r#"{"b":"b","a":"a"}"#;
    let t1: Translation = serde_json::from_str(INPUT_A).unwrap();
    let mut t2: Translation = serde_json::from_str(INPUT_B).unwrap();
    t2.apply_translation_order(&t1).unwrap();
}
#[test]
#[should_panic(expected = "self and other diverged, self being map, other value")]
fn test_order_applyance_divergence_self() {
    const INPUT_A: &str = r#"{"a":"","b":""}"#;
    const INPUT_B: &str = r#"{"b":{"b":"b"},"a":"a"}"#;
    let t1: Translation = serde_json::from_str(INPUT_A).unwrap();
    let mut t2: Translation = serde_json::from_str(INPUT_B).unwrap();
    t2.apply_translation_order(&t1).unwrap();
}
#[test]
fn test_append_unknown_keys_at_the_end_when_ordering() {
    const INPUT_A: &str = r#"{"a":"","b":""}"#;
    const INPUT_B: &str = r#"{"c":"c","a":"a"}"#;
    const EXPECTED_OUTPUT: &str = r#"{"a":"a","c":"c"}"#;
    let t1: Translation = serde_json::from_str(INPUT_A).unwrap();
    let mut t2: Translation = serde_json::from_str(INPUT_B).unwrap();
    t2.apply_translation_order(&t1).unwrap();
    assert_eq!(t2.to_string(), EXPECTED_OUTPUT);
}

#[test]
#[should_panic(expected = "This should never be called, since Vec::default() does Vec::new(), but needs T::default")]
fn test_failing_translation_default() {
    let _: Translation = Translation::default();
}
