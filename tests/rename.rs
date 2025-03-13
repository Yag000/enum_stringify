use enum_stringify::EnumStringify;
use std::str::FromStr;

#[derive(PartialEq, Debug, EnumStringify)]
enum Ainur {
    #[enum_stringify(rename = "Gods")]
    Valar,
    Maiar,
}

#[test]
fn test_simple_rename() {
    assert_eq!(Ainur::Valar.to_string(), "Gods");
    assert_eq!(Ainur::Maiar.to_string(), "Maiar");
}

#[test]
fn test_simple_rename_from_str() {
    assert_eq!(Ainur::from_str("Gods"), Ok(Ainur::Valar));
    assert_eq!(Ainur::from_str("Maiar"), Ok(Ainur::Maiar));
}

#[derive(PartialEq, Debug, EnumStringify)]
enum Ainur2 {
    #[enum_stringify(rename = "Gods")]
    Valar,
    #[enum_stringify(rename = "Raiam")]
    Maiar,
}

#[test]
fn test_simple_rename2() {
    assert_eq!(Ainur2::Valar.to_string(), "Gods");
    assert_eq!(Ainur2::Maiar.to_string(), "Raiam");
}

#[test]
fn test_simple_rename_from_str2() {
    assert_eq!(Ainur2::from_str("Gods"), Ok(Ainur2::Valar));
    assert_eq!(Ainur2::from_str("Raiam"), Ok(Ainur2::Maiar));
}

#[derive(PartialEq, Debug, EnumStringify)]
enum DoubleAniurRename {
    #[enum_stringify(rename = "Gods")]
    #[enum_stringify(rename = "Valar")]
    Valar,
    #[enum_stringify(rename = "Raiam")]
    Maiar,
}

#[test]
fn test_double_rename() {
    assert_eq!(DoubleAniurRename::Valar.to_string(), "Valar");
    assert_eq!(DoubleAniurRename::Maiar.to_string(), "Raiam");
}

#[test]
fn test_double_rename_from_str() {
    assert_eq!(
        DoubleAniurRename::from_str("Valar"),
        Ok(DoubleAniurRename::Valar)
    );
    assert_eq!(
        DoubleAniurRename::from_str("Raiam"),
        Ok(DoubleAniurRename::Maiar)
    );
}

#[derive(PartialEq, Debug, EnumStringify)]
enum Seperator {
    #[enum_stringify(rename = " ")]
    Space,
    #[enum_stringify(rename = "-")]
    Hyphen,
    #[enum_stringify(rename = "")]
    Empty,
}

#[test]
fn test_seperator_rename() {
    assert_eq!(Seperator::Space.to_string(), " ");
    assert_eq!(Seperator::Hyphen.to_string(), "-");
    assert_eq!(Seperator::Empty.to_string(), "");
}

#[test]
fn test_seperator_rename_from_str() {
    assert_eq!(Seperator::from_str(" "), Ok(Seperator::Space));
    assert_eq!(Seperator::from_str("-"), Ok(Seperator::Hyphen));
    assert_eq!(Seperator::from_str(""), Ok(Seperator::Empty));
    assert!(Seperator::from_str("|").is_err());
}

#[derive(EnumStringify, Debug, PartialEq)]
enum Istari {
    #[enum_stringify(rename = "Ólorin")]
    Gandalf,
    Saruman,
}

#[test]
fn test_rename_variants() {
    assert_eq!(Istari::Gandalf.to_string(), "Ólorin");
    assert_eq!(Istari::try_from("Ólorin").unwrap(), Istari::Gandalf);
    assert_eq!(Istari::Saruman.to_string(), "Saruman");
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper_flat")]
enum Severity {
    #[enum_stringify(rename = "critical")]
    High,
    Low,
}

#[test]
fn test_all_options() {
    assert_eq!(Severity::High.to_string(), "critical");
    assert_eq!(Severity::Low.to_string(), "PRELOWPOST");

    assert_eq!(Severity::try_from("critical").unwrap(), Severity::High);
    assert_eq!(Severity::try_from("PRELOWPOST").unwrap(), Severity::Low);
}

#[derive(EnumStringify, Debug, PartialEq)]
#[enum_stringify(prefix = "Pre", suffix = "Post", case = "upper_flat")]
enum Response {
    #[enum_stringify(rename = "okay")]
    Success,
    ErroR,
}

#[test]
fn test_combined_prefix_suffix_case_rename() {
    assert_eq!(Response::Success.to_string(), "okay");
    assert_eq!(Response::ErroR.to_string(), "PREERRORPOST");

    assert_eq!(Response::try_from("okay").unwrap(), Response::Success);
    assert_eq!(Response::try_from("PREERRORPOST").unwrap(), Response::ErroR);
}

#[derive(EnumStringify, Debug, PartialEq)]
enum SpecialChars {
    #[enum_stringify(rename = "Hello, World!")]
    HelloWorld,

    #[enum_stringify(rename = "100%Success")]
    FullSuccess,

    #[enum_stringify(rename = "Café")]
    Cafe,
}

#[test]
fn test_special_characters() {
    assert_eq!(SpecialChars::HelloWorld.to_string(), "Hello, World!");
    assert_eq!(SpecialChars::FullSuccess.to_string(), "100%Success");
    assert_eq!(SpecialChars::Cafe.to_string(), "Café");

    assert_eq!(
        SpecialChars::try_from("Hello, World!").unwrap(),
        SpecialChars::HelloWorld
    );
    assert_eq!(
        SpecialChars::try_from("100%Success").unwrap(),
        SpecialChars::FullSuccess
    );
    assert_eq!(SpecialChars::try_from("Café").unwrap(), SpecialChars::Cafe);
}

#[derive(EnumStringify, Debug, PartialEq)]
enum UnicodeEnumRename {
    #[enum_stringify(rename = "日本語")]
    Japanese,

    #[enum_stringify(rename = "🌟 Star")]
    Star,
}

#[test]
fn test_unicode_rename() {
    assert_eq!(UnicodeEnumRename::Japanese.to_string(), "日本語");
    assert_eq!(UnicodeEnumRename::Star.to_string(), "🌟 Star");

    assert_eq!(
        UnicodeEnumRename::try_from("日本語").unwrap(),
        UnicodeEnumRename::Japanese
    );
    assert_eq!(
        UnicodeEnumRename::try_from("🌟 Star").unwrap(),
        UnicodeEnumRename::Star
    );
}

#[derive(EnumStringify, Debug, PartialEq)]
enum EmptyString {
    #[enum_stringify(rename = "")]
    Silent,
}

#[test]
fn empty_rename() {
    assert_eq!(EmptyString::Silent.to_string(), "");
    assert_eq!(EmptyString::try_from("").unwrap(), EmptyString::Silent);
}
