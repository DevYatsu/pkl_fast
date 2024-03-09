#[test]
fn typealias() {
    let code = r#"x: Map<Int, String>(length <= 5) = Map(0, "0")

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

    use pkl_fast::{
        lexer::string::sanitize_code,
        prelude::{lex, parse},
    };
    let (code, str_vec) = sanitize_code(code);
    let tokens = lex(&code);
    let statements = parse(tokens, str_vec);

    assert_eq!(statements.is_ok(), true)
}
