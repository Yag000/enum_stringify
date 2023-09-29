use std::str::FromStr;

#[derive(Debug, PartialEq, enum_stringify::EnumStringify)]
#[enum_stringify(suffix = Suff)]
enum Number1 {
    Zero,
    One,
    Two,
}

#[test]
fn test_suffix_to_string() {
    assert_eq!(Number1::Zero.to_string(), "ZeroSuff");
    assert_eq!(Number1::One.to_string(), "OneSuff");
    assert_eq!(Number1::Two.to_string(), "TwoSuff");
}
#[test]
fn test_suffix_from_str() {
    assert_eq!(Number1::from_str("ZeroSuff"), Ok(Number1::Zero));
    assert_eq!(Number1::from_str("OneSuff"), Ok(Number1::One));
    assert_eq!(Number1::from_str("TwoSuff"), Ok(Number1::Two));
}

#[derive(Debug, PartialEq, enum_stringify::EnumStringify)]
#[enum_stringify(prefix = Pref)]
enum Number2 {
    Zero,
    One,
    Two,
}

#[test]
fn test_prefix_to_string() {
    assert_eq!(Number2::Zero.to_string(), "PrefZero");
    assert_eq!(Number2::One.to_string(), "PrefOne");
    assert_eq!(Number2::Two.to_string(), "PrefTwo");
}

#[test]
fn test_prefix_from_str() {
    assert_eq!(Number2::from_str("PrefZero"), Ok(Number2::Zero));
    assert_eq!(Number2::from_str("PrefOne"), Ok(Number2::One));
    assert_eq!(Number2::from_str("PrefTwo"), Ok(Number2::Two));
}

#[derive(Debug, PartialEq, enum_stringify::EnumStringify)]
#[enum_stringify(prefix = Pref, suffix = Suff)]
enum Number3 {
    Zero,
    One,
    Two,
}

#[test]
fn test_prefix_suffix_to_string() {
    assert_eq!(Number3::Zero.to_string(), "PrefZeroSuff");
    assert_eq!(Number3::One.to_string(), "PrefOneSuff");
    assert_eq!(Number3::Two.to_string(), "PrefTwoSuff");
}

#[test]
fn test_prefix_suffix_from_str() {
    assert_eq!(Number3::from_str("PrefZeroSuff"), Ok(Number3::Zero));
    assert_eq!(Number3::from_str("PrefOneSuff"), Ok(Number3::One));
    assert_eq!(Number3::from_str("PrefTwoSuff"), Ok(Number3::Two));
}

// Testing commutativity of prefix and suffix

#[derive(Debug, PartialEq, enum_stringify::EnumStringify)]
#[enum_stringify(suffix = Suff, prefix = Pref)]
enum Number4 {
    Zero,
    One,
    Two,
}

#[test]
fn test_suffix_prefix_to_string() {
    assert_eq!(Number4::Zero.to_string(), "PrefZeroSuff");
    assert_eq!(Number4::One.to_string(), "PrefOneSuff");
    assert_eq!(Number4::Two.to_string(), "PrefTwoSuff");
}

#[test]
fn test_suffix_prefix_from_str() {
    assert_eq!(Number4::from_str("PrefZeroSuff"), Ok(Number4::Zero));
    assert_eq!(Number4::from_str("PrefOneSuff"), Ok(Number4::One));
    assert_eq!(Number4::from_str("PrefTwoSuff"), Ok(Number4::Two));
}
