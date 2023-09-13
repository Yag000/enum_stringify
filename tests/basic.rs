use std::str::FromStr;

#[derive(enum_stringify::EnumStringify, Debug, PartialEq)]
enum Numbers {
    One,
    Two,
    Three,
}

#[test]
fn test_numbers_to_string() {
    assert_eq!(Numbers::One.to_string(), "One");
    assert_eq!(Numbers::Two.to_string(), "Two");
    assert_eq!(Numbers::Three.to_string(), "Three");
}

#[test]
fn test_from_str() {
    assert_eq!(Numbers::try_from("One").unwrap(), Numbers::One);
    assert_eq!(Numbers::try_from("Two").unwrap(), Numbers::Two);
    assert_eq!(Numbers::try_from("Three").unwrap(), Numbers::Three);

    assert!(Numbers::try_from("Four").is_err());
}

#[test]
fn test_from_string() {
    assert_eq!(Numbers::try_from("One".to_string()).unwrap(), Numbers::One);
    assert_eq!(Numbers::try_from("Two".to_string()).unwrap(), Numbers::Two);
    assert_eq!(
        Numbers::try_from("Three".to_string()).unwrap(),
        Numbers::Three
    );

    assert!(Numbers::try_from("Four".to_string()).is_err());
}

#[test]
fn test_from_str_trait() {
    assert_eq!(Numbers::from_str("One").unwrap(), Numbers::One);
    assert_eq!(Numbers::from_str("Two").unwrap(), Numbers::Two);
    assert_eq!(Numbers::from_str("Three").unwrap(), Numbers::Three);

    assert!(Numbers::from_str("Four").is_err());
}
