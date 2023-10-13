use std::str::FromStr;

#[derive(PartialEq, Debug, enum_stringify::EnumStringify)]
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

#[derive(PartialEq, Debug, enum_stringify::EnumStringify)]
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

#[derive(PartialEq, Debug, enum_stringify::EnumStringify)]
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
