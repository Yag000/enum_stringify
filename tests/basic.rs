use std::str::FromStr;

use enum_stringify::EnumStringify;

#[derive(EnumStringify, Debug, PartialEq)]
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

#[derive(EnumStringify, Debug, PartialEq)]
enum LargeEnum {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
}

#[test]
fn large_enum() {
    assert_eq!(LargeEnum::A1.to_string(), "A1");
    assert_eq!(LargeEnum::C10.to_string(), "C10");

    assert_eq!(LargeEnum::try_from("B5").unwrap(), LargeEnum::B5);
    assert!(LargeEnum::try_from("Z100").is_err());
}

#[derive(EnumStringify, Debug, PartialEq)]
enum FuzzyMatch {
    Alpha,
    Beta,
    Gamma,
}

#[test]
fn wrong_similar_names() {
    assert!(FuzzyMatch::try_from("alpha ").is_err()); // Extra space
    assert!(FuzzyMatch::try_from("ALPHA").is_err()); // Wrong case
    assert!(FuzzyMatch::try_from("alpHa").is_err()); // Mixed case
    assert!(FuzzyMatch::try_from("Alphaa").is_err()); // Extra character
}
