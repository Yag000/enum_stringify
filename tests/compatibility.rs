use std::str::FromStr;

// Testing compatibility with other attribute macros

#[derive(Debug, PartialEq, Eq, enum_stringify::EnumStringify, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[enum_stringify(prefix = MyPrefix, suffix = MySuffix)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn test_prefix_suffix_to_string() {
    assert_eq!(MyEnum::A.to_string(), "MyPrefixAMySuffix");
    assert_eq!(MyEnum::B.to_string(), "MyPrefixBMySuffix");
    assert_eq!(MyEnum::C.to_string(), "MyPrefixCMySuffix");
}

#[test]
fn test_prefix_suffix_from_str() {
    assert_eq!(MyEnum::from_str("MyPrefixAMySuffix"), Ok(MyEnum::A));
    assert_eq!(MyEnum::from_str("MyPrefixBMySuffix"), Ok(MyEnum::B));
    assert_eq!(MyEnum::from_str("MyPrefixCMySuffix"), Ok(MyEnum::C));
}

// Testing commutativity with other attribute macros

#[derive(Debug, PartialEq, Eq, enum_stringify::EnumStringify, serde::Serialize)]
#[enum_stringify(suffix = MySuffix, prefix = MyPrefix)]
#[serde(rename_all = "snake_case")]
enum MyEnum2 {
    A,
    B,
    C,
}

#[test]
fn test_suffix_prefix_to_string() {
    assert_eq!(MyEnum2::A.to_string(), "MyPrefixAMySuffix");
    assert_eq!(MyEnum2::B.to_string(), "MyPrefixBMySuffix");
    assert_eq!(MyEnum2::C.to_string(), "MyPrefixCMySuffix");
}

#[test]
fn test_suffix_prefix_from_str() {
    assert_eq!(MyEnum2::from_str("MyPrefixAMySuffix"), Ok(MyEnum2::A));
    assert_eq!(MyEnum2::from_str("MyPrefixBMySuffix"), Ok(MyEnum2::B));
    assert_eq!(MyEnum2::from_str("MyPrefixCMySuffix"), Ok(MyEnum2::C));
}
