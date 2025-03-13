use enum_stringify::EnumStringify;
use std::str::FromStr;

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(suffix = "Suff")]
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
#[enum_stringify(prefix = "Pref")]
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

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(prefix = "Pref", suffix = "Suff")]
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

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post")]
enum Status {
    Okk,
    Error3,
}

#[test]
fn test_prefix_suffix() {
    assert_eq!(Status::Okk.to_string(), "PreOkkPost");
    assert_eq!(Status::Error3.to_string(), "PreError3Post");

    assert_eq!(Status::try_from("PreOkkPost").unwrap(), Status::Okk);
    assert_eq!(Status::try_from("PreError3Post").unwrap(), Status::Error3);
}

// Testing commutativity of prefix, suffix and case

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(suffix = "Suff", prefix = "Pref", case = "flat")]
enum Number4 {
    Zero,
    One,
    Two,
}

#[test]
fn test_suffix_prefix_flat_to_string() {
    assert_eq!(Number4::Zero.to_string(), "prefzerosuff");
    assert_eq!(Number4::One.to_string(), "prefonesuff");
    assert_eq!(Number4::Two.to_string(), "preftwosuff");
}

#[test]
fn test_suffix_prefix_flat_from_str() {
    assert_eq!(Number4::from_str("prefzerosuff"), Ok(Number4::Zero));
    assert_eq!(Number4::from_str("prefonesuff"), Ok(Number4::One));
    assert_eq!(Number4::from_str("preftwosuff"), Ok(Number4::Two));
}

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(suffix = "Suff", prefix = "Pref", case = "upper_flat")]
enum Number5 {
    Zero,
    One,
    Two,
}

#[test]
fn test_suffix_prefix_upper_flat_to_string() {
    assert_eq!(Number5::Zero.to_string(), "PREFZEROSUFF");
    assert_eq!(Number5::One.to_string(), "PREFONESUFF");
    assert_eq!(Number5::Two.to_string(), "PREFTWOSUFF");
}

#[test]
fn test_suffix_prefix_upper_flat_from_str() {
    assert_eq!(Number5::from_str("PREFZEROSUFF"), Ok(Number5::Zero));
    assert_eq!(Number5::from_str("PREFONESUFF"), Ok(Number5::One));
    assert_eq!(Number5::from_str("PREFTWOSUFF"), Ok(Number5::Two));
}

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(case = "lower")]
enum Number6 {
    Zero,
    One,
    Two,
}

#[test]
fn test_lower_to_string() {
    assert_eq!(Number6::Zero.to_string(), "zero");
    assert_eq!(Number6::One.to_string(), "one");
    assert_eq!(Number6::Two.to_string(), "two");
}

#[test]
fn test_lower_from_str() {
    assert_eq!(Number6::from_str("zero"), Ok(Number6::Zero));
    assert_eq!(Number6::from_str("one"), Ok(Number6::One));
    assert_eq!(Number6::from_str("two"), Ok(Number6::Two));
}

#[derive(Debug, PartialEq, EnumStringify)]
#[enum_stringify(case = "upper")]
enum Number7 {
    Zero,
    One,
    Two,
}

#[test]
fn test_upper_to_string() {
    assert_eq!(Number7::Zero.to_string(), "ZERO");
    assert_eq!(Number7::One.to_string(), "ONE");
    assert_eq!(Number7::Two.to_string(), "TWO");
}

#[test]
fn test_upper_from_str() {
    assert_eq!(Number7::from_str("ZERO"), Ok(Number7::Zero));
    assert_eq!(Number7::from_str("ONE"), Ok(Number7::One));
    assert_eq!(Number7::from_str("TWO"), Ok(Number7::Two));
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "😀", suffix = "🥳")]
enum UnicodeEnum {
    Japanese,
    Star,
}

#[test]
fn test_unicode() {
    assert_eq!(UnicodeEnum::Japanese.to_string(), "😀Japanese🥳");
    assert_eq!(UnicodeEnum::Star.to_string(), "😀Star🥳");

    assert_eq!(
        UnicodeEnum::try_from("😀Japanese🥳").unwrap(),
        UnicodeEnum::Japanese
    );
    assert_eq!(
        UnicodeEnum::try_from("😀Star🥳").unwrap(),
        UnicodeEnum::Star
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "", suffix = "")]
enum Punctuated {
    Excited,
    Happy,
}

#[test]
fn test_empty_suffix_prefix() {
    assert_eq!(Punctuated::Excited.to_string(), "Excited");
    assert_eq!(Punctuated::Happy.to_string(), "Happy");

    assert_eq!(
        Punctuated::try_from("Excited").unwrap(),
        Punctuated::Excited
    );
    assert_eq!(Punctuated::try_from("Happy").unwrap(), Punctuated::Happy);
}
