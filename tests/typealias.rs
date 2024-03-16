#[test]
fn typealias() {
    let source = r#"x: Map<Int, String>(length <= 5) = Map(0, "0")

class Bird {
  name: String(length >= 3)   
  parent: String(this != name)  
}

emailRegex: Regex = Regex("test")


email: EmailAddress = "pigeon@example.com"

emailList: List<EmailAddress> = List("pigeon@example.com", "parrot@example.com")
typealias EmailList = List<EmailAddress>

emailList: EmailList = List("pigeon@example.com", "parrot@example.com")


typealias StringMap<Value> = Map<String, Value>

map: StringMap<Int> = Map("Pigeon", 42, "Falcon", 21)



typealias Hey = "hey"
hey_string: Hey = "hey"

typealias Foo = "foo"|"bar"|"baz"
"#;

    use pkl_fast::prelude::parse;
    let statements = parse("main.pkl", source);

    assert_eq!(statements.is_ok(), true)
}
