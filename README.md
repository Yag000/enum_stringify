# enum-stringify

Set of macros (only one for now) to generate a string representation of an enum. When using
`#[derive(EnumStringify)]` on an enum, it will implement `std::fmt::Display`, `TryFrom<&str>`,
`TryFrom<String>` and `std::str::FromStr` for it. It will use the name of the enum variant as the
string representation.

## Usage

```rust
use enum_stringify::EnumStringify;

#[derive(EnumStringify)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
    println!("{}", MyEnum::Variant1); // Prints "Variant1"
    assert_eq!(MyEnum::Variant1.to_string(), "Variant1");
    assert_eq!(MyEnum::try_from("Variant2").unwrap(), MyEnum::Variant2);
    assert_eq!(MyEnum::try_from("Variant3".to_string()).unwrap(), MyEnum::Variant3);
    assert_eq!(MyEnum::from_str("Variant1").unwrap(), MyEnum::Variant1);
}
```

## Documentation and installation

See [docs.rs](https://docs.rs/enum-stringify) for documentation.
It is available on [crates.io](https://crates.io/crates/enum-stringify) as well.
