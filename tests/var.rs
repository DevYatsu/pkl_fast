use pkl_fast::prelude::parse;

#[test]
fn floats_var() {
    const SOURCE: &str = "num1 = .23
num2 = 1.23
num3 = 1.23e2 
num4 = 1.23e-2
notANumber = NaN
positiveInfinity = Infinity
negativeInfinity = -Infinity";

    let statements = parse("main.pkl", SOURCE);

    assert_eq!(statements.is_ok(), true);
}

#[test]
fn ints_var() {
    const SOURCE: &str = "num1 = 123
num2 = 0x012AFF 
num3 = 0b00010111 
num4 = 0o755 
num1 = 1_000_000 
num2 = 0x0134_64DE 
num3 = 0b0001_0111 
num4 = 0o0134_6475";

    let statements = parse("main.pkl", SOURCE);

    assert_eq!(statements.is_ok(), true);
}

#[test]
fn duration_and_datasize() {
    const SOURCE: &str = "duration1 = 5.ns  // nanoseconds (smallest unit)
duration2 = 5.us  // microseconds
duration3 = 5.ms  // milliseconds
duration4 = 5.s   // seconds
duration5 = 5.min // minutes
duration6 = 5.h   // hours
duration7 = 3.d   // days (largest unit)
duration8 = 1.6666666666666667.min

datasize1 = 5.b  // bytes (smallest unit)
datasize2 = 5.kb // kilobytes
datasize3 = 5.mb // megabytes
datasize4 = 5.gb // gigabytes
datasize5 = 5.tb // terabytes
datasize6 = 5.pb // petabytes (largest unit)
datasize1 = 5.b   // bytes (smallest unit)
datasize2 = 5.kib // kibibytes
datasize3 = 5.mib // mebibytes
datasize4 = 5.gib // gibibytes
datasize5 = 5.tib // tebibytes
datasize6 = 5.pib // pebibytes (largest unit)
datasize7 = 1.6666666666666667.mb
";

    let statements = parse("main.pkl", SOURCE);

    assert_eq!(statements.is_ok(), true);
}

#[test]
fn str_var() {
    const SOURCE: &str = r#"ENV_VALUE="TEST_VALUE"
s="str"
greeting = "Hi,\u{1F60A} \t my friend! \u{1E0A} \n How are you? \n \(ENV_VALUE + s)"

name = "Dodo"
greeting = "Hi, \(name)!" 
str = "\(x + 2) plus \(x * 2) is \(0x80) \t " 

bird = "Dodo"
local message = """
Although the \(bird) is ex\ntinct, \u{a2e3e}
the species will be remembered. \\
"""

"#;

    let statements = parse("main.pkl", SOURCE);

    assert_eq!(statements.is_ok(), true);
}
