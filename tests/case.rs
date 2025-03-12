use enum_stringify::EnumStringify;

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "upper")]
enum Letters {
    A,
    B,
}

#[test]
fn test_case_conversion_upper() {
    assert_eq!(Letters::A.to_string(), "A");
    assert_eq!(Letters::B.to_string(), "B");

    assert_eq!(Letters::try_from("A").unwrap(), Letters::A);
    assert_eq!(Letters::try_from("B").unwrap(), Letters::B);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "lower")]
enum Colors {
    Red,
    Blue,
}

#[test]
fn test_case_conversion_lower() {
    assert_eq!(Colors::Red.to_string(), "red");
    assert_eq!(Colors::Blue.to_string(), "blue");

    assert_eq!(Colors::try_from("red").unwrap(), Colors::Red);
    assert_eq!(Colors::try_from("blue").unwrap(), Colors::Blue);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(case = "upper_flat")]
enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

#[test]
fn test_upper_flat_case() {
    assert_eq!(Season::Spring.to_string(), "SPRING");
    assert_eq!(Season::Summer.to_string(), "SUMMER");
    assert_eq!(Season::Fall.to_string(), "FALL");
    assert_eq!(Season::Winter.to_string(), "WINTER");

    assert_eq!(Season::try_from("SPRING").unwrap(), Season::Spring);
    assert_eq!(Season::try_from("SUMMER").unwrap(), Season::Summer);
    assert_eq!(Season::try_from("FALL").unwrap(), Season::Fall);
    assert_eq!(Season::try_from("WINTER").unwrap(), Season::Winter);
}
