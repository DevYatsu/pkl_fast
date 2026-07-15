use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;
use pkl_core::*;
use pkl_core::decode::decode_response;

// ── Primitive PklDecode tests (ported from pkl-go decoder_test.go/unmarshal_test.go) ──

#[test]
fn test_decode_null() {
    let result = <Option<i64>>::decode(Value::Null).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_decode_bool() {
    assert!(bool::decode(Value::Boolean(true)).unwrap());
    assert!(!bool::decode(Value::Boolean(false)).unwrap());
}

#[test]
fn test_decode_bool_error() {
    let err = bool::decode(Value::Int(1)).unwrap_err();
    assert!(matches!(err, PklError::TypeMismatch { .. }));
}

#[test]
fn test_decode_int() {
    assert_eq!(i64::decode(Value::Int(42)).unwrap(), 42);
    assert_eq!(i64::decode(Value::Int(-1)).unwrap(), -1);
    assert_eq!(i64::decode(Value::Int(0)).unwrap(), 0);
    assert_eq!(i64::decode(Value::Int(i64::MAX)).unwrap(), i64::MAX);
    assert_eq!(i64::decode(Value::Int(i64::MIN)).unwrap(), i64::MIN);
}

#[test]
fn test_decode_int_small_types() {
    assert_eq!(i32::decode(Value::Int(42)).unwrap(), 42i32);
    assert_eq!(i16::decode(Value::Int(42)).unwrap(), 42i16);
    assert_eq!(i8::decode(Value::Int(42)).unwrap(), 42i8);
}

#[test]
fn test_decode_uint() {
    assert_eq!(u64::decode(Value::Int(42)).unwrap(), 42u64);
    assert_eq!(u32::decode(Value::Int(42)).unwrap(), 42u32);
    assert_eq!(u16::decode(Value::Int(42)).unwrap(), 42u16);
    assert_eq!(u8::decode(Value::Int(42)).unwrap(), 42u8);
}

#[test]
fn test_decode_float() {
    let f = f64::decode(Value::Float(3.14)).unwrap();
    assert!((f - 3.14).abs() < 1e-10);
    // Ints can decode as floats
    let f = f64::decode(Value::Int(42)).unwrap();
    assert!((f - 42.0).abs() < 1e-10);
}

#[test]
fn test_decode_string() {
    assert_eq!(String::decode(Value::String("hello".into())).unwrap(), "hello");
    assert_eq!(String::decode(Value::String("".into())).unwrap(), "");
}

#[test]
fn test_decode_string_error() {
    let err = String::decode(Value::Int(42)).unwrap_err();
    assert!(matches!(err, PklError::TypeMismatch { .. }));
}

#[test]
fn test_decode_duration() {
    let d = Duration::decode(Value::Duration(Duration::new(30.0, "s"))).unwrap();
    assert_eq!(d.value, 30.0);
    assert_eq!(d.unit, "s");
}

#[test]
fn test_decode_datasize() {
    let ds = DataSize::decode(Value::DataSize(DataSize::new(100.0, "gb"))).unwrap();
    assert_eq!(ds.value, 100.0);
    assert_eq!(ds.unit, "gb");
}

// ── Collection decode tests ──

#[test]
fn test_decode_vec() {
    let items = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    let result = Vec::<i64>::decode(Value::List(items)).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_decode_vec_from_listing() {
    let items = vec![Value::Int(1), Value::Int(2)];
    let result = Vec::<i64>::decode(Value::Listing(items)).unwrap();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_decode_vec_from_set() {
    let items = vec![Value::Int(1), Value::Int(2)];
    let result = Vec::<i64>::decode(Value::Set(items)).unwrap();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_decode_btreemap() {
    let mut map = BTreeMap::new();
    map.insert(Value::String("a".into()), Value::Int(1));
    map.insert(Value::String("b".into()), Value::Int(2));
    let result = BTreeMap::<String, i64>::decode(Value::Mapping(map)).unwrap();
    let mut expected = BTreeMap::new();
    expected.insert("a".into(), 1i64);
    expected.insert("b".into(), 2i64);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_btreemap_from_map() {
    let pairs = vec![
        (Value::String("a".into()), Value::Int(1)),
        (Value::String("b".into()), Value::Int(2)),
    ];
    let result = BTreeMap::<String, i64>::decode(Value::Map(pairs)).unwrap();
    let mut expected = BTreeMap::new();
    expected.insert("a".into(), 1i64);
    expected.insert("b".into(), 2i64);
    assert_eq!(result, expected);
}

#[test]
fn test_decode_option_some() {
    let result = Option::<i64>::decode(Value::Int(42)).unwrap();
    assert_eq!(result, Some(42));
}

#[test]
fn test_decode_option_none() {
    let result = Option::<i64>::decode(Value::Null).unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_decode_option_nested() {
    let result = Option::<Option<i64>>::decode(Value::Null).unwrap();
    assert_eq!(result, None);
    let result = Option::<Option<i64>>::decode(Value::Int(42)).unwrap();
    assert_eq!(result, Some(Some(42)));
}

#[test]
fn test_decode_nested_vec() {
    let items = vec![
        Value::List(vec![Value::Int(1), Value::Int(2)]),
        Value::List(vec![Value::Int(3), Value::Int(4)]),
    ];
    let result = Vec::<Vec<i64>>::decode(Value::List(items)).unwrap();
    assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
}

// ── Object/Struct decode tests ──

#[test]
fn test_decode_object() {
    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("Pigeon".into()));
    props.insert("age".into(), Value::Int(8));
    let value = Value::Object(props);

    let mut map = BTreeMap::<String, Value>::decode(value).unwrap();
    assert_eq!(
        map.remove("name").unwrap().as_str().unwrap(),
        "Pigeon"
    );
    assert_eq!(map.remove("age").unwrap().as_int().unwrap(), 8);
}

#[test]
fn test_decode_object_with_nested() {
    let mut inner = BTreeMap::new();
    inner.insert("diet".into(), Value::String("Seeds".into()));
    let mut outer = BTreeMap::new();
    outer.insert("name".into(), Value::String("Pigeon".into()));
    outer.insert("attributes".into(), Value::Object(inner));

    let result = Value::Object(outer);
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("name").unwrap().as_str().unwrap(), "Pigeon");
    let attrs = obj.get("attributes").unwrap().as_object().unwrap();
    assert_eq!(attrs.get("diet").unwrap().as_str().unwrap(), "Seeds");
}

// ── Error case tests ──

#[test]
fn test_decode_type_mismatch_error() {
    let err = i64::decode(Value::String("not a number".into())).unwrap_err();
    match err {
        PklError::TypeMismatch { expected, actual } => {
            assert_eq!(expected, "Int");
            assert!(actual.contains("String"));
        }
        _ => panic!("Expected TypeMismatch error"),
    }
}

// ── PklDecode derive macro tests ──

#[test]
fn test_derive_simple() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Bird {
        name: String,
        lifespan: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("Pigeon".into()));
    props.insert("lifespan".into(), Value::Int(8));

    let bird = Bird::decode(Value::Object(props)).unwrap();
    assert_eq!(bird.name, "Pigeon");
    assert_eq!(bird.lifespan, 8);
}

#[test]
fn test_derive_with_bool() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        debug: bool,
        verbose: bool,
    }

    let mut props = BTreeMap::new();
    props.insert("debug".into(), Value::Boolean(true));
    props.insert("verbose".into(), Value::Boolean(false));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert!(cfg.debug);
    assert!(!cfg.verbose);
}

#[test]
fn test_derive_with_float() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Measurement {
        value: f64,
        tolerance: f64,
    }

    let mut props = BTreeMap::new();
    props.insert("value".into(), Value::Float(3.14));
    props.insert("tolerance".into(), Value::Float(0.01));

    let m = Measurement::decode(Value::Object(props)).unwrap();
    assert!((m.value - 3.14).abs() < 1e-10);
    assert!((m.tolerance - 0.01).abs() < 1e-10);
}

#[test]
fn test_derive_missing_field_error() {
    #[derive(PklDecode, Debug)]
    struct Bird {
        name: String,
        age: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("Pigeon".into()));
    // "age" is missing

    let err = Bird::decode(Value::Object(props)).unwrap_err();
    match err {
        PklError::MissingProperty(field) => assert_eq!(field, "age"),
        _ => panic!("Expected MissingProperty error"),
    }
}

#[test]
fn test_derive_extra_fields_ignored() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Bird {
        name: String,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("Pigeon".into()));
    props.insert("extra".into(), Value::Int(99)); // ignored

    let bird = Bird::decode(Value::Object(props)).unwrap();
    assert_eq!(bird.name, "Pigeon");
}

#[test]
fn test_derive_wrong_type_error() {
    #[derive(PklDecode, Debug)]
    struct Bird {
        name: i64, // wrong type — Pkl has String
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("Pigeon".into()));

    let err = Bird::decode(Value::Object(props)).unwrap_err();
    assert!(matches!(err, PklError::TypeMismatch { .. }));
}

#[test]
fn test_derive_camel_case_conversion() {
    // Pkl uses camelCase but Rust uses snake_case
    #[derive(PklDecode, Debug, PartialEq)]
    struct AppConfig {
        db_host: String,
        max_connections: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("dbHost".into(), Value::String("localhost".into()));
    props.insert("maxConnections".into(), Value::Int(100));

    let cfg = AppConfig::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.db_host, "localhost");
    assert_eq!(cfg.max_connections, 100);
}

// ── #[pkl(rename)] attribute tests ──

#[test]
fn test_pkl_rename() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        #[pkl(rename = "db_host")]
        dbHost: String,
    }

    let mut props = BTreeMap::new();
    props.insert("db_host".into(), Value::String("localhost".into()));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.dbHost, "localhost");
}

#[test]
fn test_pkl_rename_overrides_camel_case() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        #[pkl(rename = "originalName")]
        my_field: String,
    }

    let mut props = BTreeMap::new();
    props.insert("originalName".into(), Value::String("value".into()));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.my_field, "value");
}

// ── #[pkl(default)] attribute tests ──

#[test]
fn test_pkl_default_present() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(default)]
        timeout: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("test".into()));
    props.insert("timeout".into(), Value::Int(30));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "test");
    assert_eq!(cfg.timeout, 30);
}

#[test]
fn test_pkl_default_missing() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(default)]
        timeout: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("test".into()));
    // "timeout" is missing

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "test");
    assert_eq!(cfg.timeout, 0); // Default for i64
}

#[test]
fn test_pkl_default_with_string() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        #[pkl(default)]
        label: String,
    }

    let props = BTreeMap::new(); // empty

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.label, ""); // Default for String
}

// ── #[pkl(skip)] attribute tests ──

#[test]
fn test_pkl_skip() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(skip)]
        computed: String,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("test".into()));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "test");
    assert_eq!(cfg.computed, "");
}

#[test]
fn test_pkl_skip_with_default_value() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(skip)]
        count: i64,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("x".into()));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "x");
    assert_eq!(cfg.count, 0);
}

// ── Combined attribute tests ──

#[test]
fn test_pkl_rename_and_default() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        #[pkl(rename = "timeoutMs", default)]
        timeout: i64,
    }

    // Missing — should use default
    let cfg = Config::decode(Value::Object(BTreeMap::new())).unwrap();
    assert_eq!(cfg.timeout, 0);

    // Present with renamed key
    let mut props = BTreeMap::new();
    props.insert("timeoutMs".into(), Value::Int(42));
    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.timeout, 42);
}

// ── HashMap/HashSet/Box/Arc PklDecode tests ──

#[test]
fn test_decode_hashmap() {
    let mut map = BTreeMap::new();
    map.insert(Value::String("a".into()), Value::Int(1));
    let val = Value::Mapping(map);
    let result: HashMap<String, i64> = HashMap::decode(val).unwrap();
    assert_eq!(result.get("a"), Some(&1));
}

#[test]
fn test_decode_hashset() {
    let items = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    let val = Value::Set(items);
    let result: HashSet<i64> = HashSet::decode(val).unwrap();
    assert_eq!(result.len(), 3);
    assert!(result.contains(&1));
}

#[test]
fn test_decode_box() {
    let val = Value::Int(42);
    let result: Box<i64> = Box::decode(val).unwrap();
    assert_eq!(*result, 42);
}

#[test]
fn test_decode_arc() {
    let val = Value::String("hello".into());
    let result: Arc<String> = Arc::decode(val).unwrap();
    assert_eq!(*result, "hello");
}

// ── Enum tests ──

#[test]
fn test_enum_string_literal_unit() {
    #[derive(PklDecode, Debug, PartialEq)]
    enum Diet {
        Seeds,
        Berries,
        Insects,
    }

    assert_eq!(Diet::decode(Value::String("Seeds".into())).unwrap(), Diet::Seeds);
    assert_eq!(Diet::decode(Value::String("Berries".into())).unwrap(), Diet::Berries);
    assert_eq!(Diet::decode(Value::String("Insects".into())).unwrap(), Diet::Insects);
}

#[test]
fn test_enum_string_literal_error() {
    #[derive(PklDecode, Debug, PartialEq)]
    enum Diet {
        Seeds,
        Berries,
    }

    let err = Diet::decode(Value::String("Unknown".into())).unwrap_err();
    assert!(matches!(err, PklError::DecodeError(_)));
}

#[test]
fn test_enum_unnamed_variants() {
    #[derive(PklDecode, Debug, PartialEq)]
    enum MyVal {
        Int(i64),
        Float(f64),
        Text(String),
    }

    assert_eq!(MyVal::decode(pkl_core::Value::Int(42)).unwrap(), MyVal::Int(42));
    assert_eq!(MyVal::decode(pkl_core::Value::Float(3.14)).unwrap(), MyVal::Float(3.14));
    assert_eq!(MyVal::decode(pkl_core::Value::String("hi".into())).unwrap(), MyVal::Text("hi".into()));
}

#[test]
fn test_enum_mixed_variants_try_order() {
    #[derive(PklDecode, Debug, PartialEq)]
    enum Num {
        Int(i64),
        Float(f64),
    }

    // Int should match the Int variant first (not Float)
    assert_eq!(Num::decode(pkl_core::Value::Int(42)).unwrap(), Num::Int(42));
}

#[test]
fn test_enum_with_nested_struct() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Inner {
        name: String,
    }

    #[derive(PklDecode, Debug, PartialEq)]
    enum Container {
        Simple(String),
        Complex(Inner),
    }

    // String variant
    assert_eq!(
        Container::decode(pkl_core::Value::String("hello".into())).unwrap(),
        Container::Simple("hello".into())
    );

    // Struct variant (object)
    let mut props = BTreeMap::new();
    props.insert("name".into(), pkl_core::Value::String("test".into()));
    let result = Container::decode(pkl_core::Value::Object(props)).unwrap();
    match result {
        Container::Complex(inner) => assert_eq!(inner.name, "test"),
        _ => panic!("expected Complex variant"),
    };
}

// ── #[pkl(flatten)] tests ──

#[test]
fn test_pkl_flatten_extra_props() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(flatten)]
        extra: BTreeMap<String, Value>,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("test".into()));
    props.insert("unexpected".into(), Value::Int(42));
    props.insert("another".into(), Value::Boolean(true));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "test");
    assert_eq!(cfg.extra.len(), 2);
    assert_eq!(cfg.extra.get("unexpected").unwrap().as_int(), Some(42));
    assert_eq!(cfg.extra.get("another").unwrap().as_bool(), Some(true));
}

#[test]
fn test_pkl_flatten_no_extra() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        name: String,
        #[pkl(flatten)]
        extra: BTreeMap<String, Value>,
    }

    let mut props = BTreeMap::new();
    props.insert("name".into(), Value::String("test".into()));

    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "test");
    assert!(cfg.extra.is_empty());
}

#[test]
fn test_pkl_flatten_with_rename_and_default() {
    #[derive(PklDecode, Debug, PartialEq)]
    struct Config {
        #[pkl(rename = "dbName", default)]
        name: String,
        #[pkl(flatten)]
        extra: BTreeMap<String, Value>,
    }

    // No "dbName" — uses default
    let mut props = BTreeMap::new();
    props.insert("custom".into(), Value::Int(99));
    let cfg = Config::decode(Value::Object(props)).unwrap();
    assert_eq!(cfg.name, "");
    assert_eq!(cfg.extra.get("custom").unwrap().as_int(), Some(99));
}

// ── Tagged union enum tests ──

#[test]
fn test_tagged_enum_unit_variants() {
    #[derive(PklDecode, Debug, PartialEq)]
    #[pkl(tag = "type")]
    enum Animal {
        Bird,
        Cat,
        Dog,
    }

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("Bird".into()));
    assert_eq!(Animal::decode(Value::Object(props)).unwrap(), Animal::Bird);

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("Cat".into()));
    assert_eq!(Animal::decode(Value::Object(props)).unwrap(), Animal::Cat);
}

#[test]
fn test_tagged_enum_unnamed_variants() {
    #[derive(PklDecode, Debug, PartialEq)]
    #[pkl(tag = "kind")]
    enum Shape {
        Circle(f64),
        #[pkl(tag = "rect")]
        Rectangle(f64, f64),
    }

    let mut props = BTreeMap::new();
    props.insert("kind".into(), Value::String("Circle".into()));
    props.insert("value".into(), Value::Float(5.0));

    // The value is passed as the full object, which Circle tries to decode
    // Circle(f64) tries to decode the object as f64, which fails
    // This test verifies the dispatch works
    let result = Shape::decode(Value::Object(props));
    // Circle tries decode the object as f64, which fails for an object
    assert!(result.is_err());
}

#[test]
fn test_tagged_enum_named_variants() {
    #[derive(PklDecode, Debug, PartialEq)]
    #[pkl(tag = "type")]
    enum Shape {
        #[pkl(tag = "circle")]
        Circle { radius: f64 },
        #[pkl(tag = "rect")]
        Rectangle { width: f64, height: f64 },
    }

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("circle".into()));
    props.insert("radius".into(), Value::Float(2.5));
    let result = Shape::decode(Value::Object(props)).unwrap();
    match result {
        Shape::Circle { radius } => assert!((radius - 2.5).abs() < 1e-10),
        _ => panic!("expected Circle"),
    }

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("rect".into()));
    props.insert("width".into(), Value::Float(3.0));
    props.insert("height".into(), Value::Float(4.0));
    let result = Shape::decode(Value::Object(props)).unwrap();
    match result {
        Shape::Rectangle { width, height } => {
            assert!((width - 3.0).abs() < 1e-10);
            assert!((height - 4.0).abs() < 1e-10);
        }
        _ => panic!("expected Rectangle"),
    }
}

#[test]
fn test_tagged_enum_named_missing_field() {
    #[derive(PklDecode, Debug, PartialEq)]
    #[pkl(tag = "type")]
    enum Shape {
        Circle { radius: f64 },
    }

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("Circle".into()));
    // Missing "radius" field
    let err = Shape::decode(Value::Object(props)).unwrap_err();
    assert!(matches!(err, PklError::MissingProperty(_)));
}

#[test]
fn test_tagged_enum_unknown_tag() {
    #[derive(PklDecode, Debug, PartialEq)]
    #[pkl(tag = "type")]
    enum Animal {
        Bird,
        Cat,
    }

    let mut props = BTreeMap::new();
    props.insert("type".into(), Value::String("Fish".into()));
    let err = Animal::decode(Value::Object(props)).unwrap_err();
    assert!(matches!(err, PklError::DecodeError(_)));
}

// ── Cow<str> tests ──

#[test]
fn test_decode_cow_str() {
    let val = Value::String("hello".into());
    let result: Cow<'_, str> = PklDecode::decode(val).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_decode_cow_str_from_int() {
    let val = Value::Int(42);
    let err = <Cow<'_, str> as PklDecode>::decode(val).unwrap_err();
    assert!(matches!(err, PklError::TypeMismatch { .. }));
}

// ── Array [T; N] tests ──

#[test]
fn test_decode_array_3() {
    let items = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    let val = Value::List(items);
    let result: [i64; 3] = <[i64; 3]>::decode(val).unwrap();
    assert_eq!(result, [1, 2, 3]);
}

#[test]
fn test_decode_array_0() {
    let val = Value::List(vec![]);
    let result: [i64; 0] = <[i64; 0]>::decode(val).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_decode_array_1() {
    let items = vec![Value::Int(42)];
    let val = Value::List(items);
    let result: [i64; 1] = <[i64; 1]>::decode(val).unwrap();
    assert_eq!(result[0], 42);
}

#[test]
fn test_decode_array_wrong_length() {
    let items = vec![Value::Int(1), Value::Int(2)];
    let val = Value::List(items);
    let err = <[i64; 3]>::decode(val).unwrap_err();
    assert!(matches!(err, PklError::DecodeError(_)));
}

#[test]
fn test_decode_array_nested() {
    let inner = vec![
        Value::List(vec![Value::Int(1), Value::Int(2)]),
        Value::List(vec![Value::Int(3), Value::Int(4)]),
    ];
    let val = Value::List(inner);
    let result: [[i64; 2]; 2] = <[[i64; 2]; 2]>::decode(val).unwrap();
    assert_eq!(result, [[1, 2], [3, 4]]);
}

// ── Fuzz / robustness tests: ensure decode_response never panics ──

#[test]
fn test_decode_fuzz_empty() {
    // Empty input
    let result = decode_response(b"");
    assert!(result.is_err());
}

#[test]
fn test_decode_fuzz_garbage() {
    let garbage: Vec<Vec<u8>> = vec![
        b"hello world".to_vec(),
        b"\xff\xff\xff\xff".to_vec(),
        b"\x00\x00\x00\x00".to_vec(),
        b"\xc1\xc1\xc1\xc1".to_vec(),
        vec![0x00u8; 64],
        vec![0xffu8; 64],
        vec![0x92, 0x20], // array header but truncated
        vec![0x92, 0x20, 0x81], // Map with truncated content
        vec![0x92, 0x20, 0xc0], // nil inside message
        vec![0x93, 0x01, 0x02], // 3-element array, truncated
    ];
    for input in &garbage {
        // Must never panic — both Ok and Err are fine for garbage input
        let _ = decode_response(input);
    }
}

#[test]
fn test_decode_fuzz_oversized_integers() {
    let inputs: Vec<Vec<u8>> = vec![
        vec![0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], // max u64
        vec![0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // min i64
    ];
    for input in &inputs {
        let result = decode_response(input);
        assert!(result.is_err() || result.is_ok(), "unexpected result for {:?}", input);
    }
}

#[test]
fn test_decode_fuzz_deep_nesting() {
    // Build a deeply nested array that should hit the depth limit
    let mut msgpack = vec![0x92u8, 0x20]; // [0x20, ...
    for _ in 0..200 {
        msgpack.extend_from_slice(&[0x92u8, 0x20]); // nest deeper
    }
    let result = decode_response(&msgpack);
    assert!(result.is_err(), "deep nesting should hit depth limit");
}

#[test]
fn test_decode_fuzz_random_bytes() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = seed;
    for _ in 0..100 {
        let len = (rng % 256) as usize;
        let mut bytes = vec![0u8; len];
        for b in &mut bytes {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *b = (rng >> 32) as u8;
        }
        let result = decode_response(&bytes);
        // Should never panic — Err is fine for garbage
        let _ = result;
    }
}
