use pkl_core::{DataSize, Duration, Value};

// ── Duration conversion tests ──

#[test]
fn test_duration_to_nanos() {
    assert_eq!(Duration::new(5.0, "ns").to_nanos(), 5.0);
    assert_eq!(Duration::new(5.0, "us").to_nanos(), 5_000.0);
    assert_eq!(Duration::new(5.0, "ms").to_nanos(), 5_000_000.0);
    assert_eq!(Duration::new(5.0, "s").to_nanos(), 5_000_000_000.0);
    assert_eq!(Duration::new(5.0, "min").to_nanos(), 300_000_000_000.0);
    assert_eq!(Duration::new(5.0, "h").to_nanos(), 18_000_000_000_000.0);
    assert_eq!(Duration::new(3.0, "d").to_nanos(), 259_200_000_000_000.0);
}

// ── DataSize conversion tests ──

#[test]
fn test_datasize_to_bytes() {
    assert_eq!(DataSize::new(1.0, "b").to_bytes(), 1.0);
    assert_eq!(DataSize::new(5.0, "kb").to_bytes(), 5_000.0);
    assert_eq!(DataSize::new(5.0, "mb").to_bytes(), 5_000_000.0);
    assert_eq!(DataSize::new(5.0, "gb").to_bytes(), 5_000_000_000.0);
    assert_eq!(DataSize::new(5.0, "tb").to_bytes(), 5_000_000_000_000.0);
    assert_eq!(DataSize::new(5.0, "pb").to_bytes(), 5_000_000_000_000_000.0);
    assert_eq!(DataSize::new(5.0, "kib").to_bytes(), 5_120.0);
    assert_eq!(DataSize::new(5.0, "mib").to_bytes(), 5_242_880.0);
    assert_eq!(DataSize::new(5.0, "gib").to_bytes(), 5_368_709_120.0);
    assert_eq!(DataSize::new(5.0, "tib").to_bytes(), 5_497_558_138_880.0);
    assert_eq!(DataSize::new(5.0, "pib").to_bytes(), 5_629_499_534_213_120.0);
}

// ── Value downcast tests ──

#[test]
fn test_value_downcast_bool() {
    assert_eq!(Value::Boolean(true).as_bool(), Some(true));
    assert_eq!(Value::Boolean(false).as_bool(), Some(false));
    assert_eq!(Value::Null.as_bool(), None);
}

#[test]
fn test_value_downcast_int() {
    assert_eq!(Value::Int(42).as_int(), Some(42));
    assert_eq!(Value::Int(-1).as_int(), Some(-1));
    assert_eq!(Value::Null.as_int(), None);
}

#[test]
fn test_value_downcast_float() {
    match Value::Float(3.14).as_float() {
        Some(f) => assert!((f - 3.14).abs() < 1e-10),
        None => panic!("expected Some float"),
    }
    assert_eq!(Value::Null.as_float(), None);
}

#[test]
fn test_value_downcast_str() {
    assert_eq!(Value::String("hello".into()).as_str(), Some("hello"));
    assert_eq!(Value::Null.as_str(), None);
}

#[test]
fn test_value_downcast_object() {
    let mut map = std::collections::BTreeMap::new();
    map.insert("key".into(), Value::Int(1));
    let obj = Value::Object(map);
    assert!(obj.as_object().is_some());
    assert_eq!(Value::Null.as_object(), None);
}

#[test]
fn test_value_downcast_list() {
    let list = Value::List(vec![Value::Int(1), Value::Int(2)]);
    assert!(list.as_list().is_some());
    let listing = Value::Listing(vec![Value::Int(1)]);
    assert!(listing.as_list().is_some());
    assert_eq!(Value::Null.as_list(), None);
}
