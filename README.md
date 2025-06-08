# enum-stringify

[![Crates.io](https://img.shields.io/crates/v/enum-stringify.svg)](https://crates.io/crates/enum-stringify)
[![docs.rs](https://img.shields.io/badge/docs.rs-enum--stringify-blue)](https://docs.rs/enum-stringify)
![License](https://img.shields.io/crates/l/enum-stringify)

A procedural macro crate to generate string representations of Rust enums.  
Derive `EnumStringify` to automatically implement `Display`, `TryFrom<&str>`, `TryFrom<String>`, and `FromStr` for your enum, using the variant name as the string representation.

---

## Features

- **Automatic string conversion:** Implements `Display`, `FromStr`, and `TryFrom` for enums.
- **Customizable output:** Add prefixes, suffixes, and change the case of the string representation.
- **Ergonomic usage:** Simple derive macro with useful defaults.

---

## Usage

```rust
use enum_stringify::EnumStringify;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(EnumStringify)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

fn main() {
    // Display
    println!("{}", MyEnum::Variant1); // Prints "Variant1"

    // To string
    assert_eq!(MyEnum::Variant1.to_string(), "Variant1");

    // TryFrom<&str>
    assert_eq!(MyEnum::try_from("Variant2").unwrap(), MyEnum::Variant2);

    // TryFrom<String>
    assert_eq!(MyEnum::try_from("Variant3".to_string()).unwrap(), MyEnum::Variant3);

    // FromStr
    assert_eq!(MyEnum::from_str("Variant1").unwrap(), MyEnum::Variant1);
}
```

---

## Custom String Representation

You can customize the string representation using attributes for prefix, suffix, and case.

```rust
use enum_stringify::EnumStringify;

#[derive(EnumStringify)]
#[enum_stringify(prefix = "MyPrefix", suffix = "MySuffix", case = "upper_flat")]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}
```

In this example, the string representation of `MyEnum::Variant1` will be `MyPrefixVARIANT1MySuffix`.

### Supported options

- **prefix:** String prepended to each variant
- **suffix:** String appended to each variant
- **case:** Changes the case (`"upper_flat"`, `"lower_flat"`, etc.)

---

## Documentation

- [API Documentation on docs.rs](https://docs.rs/enum-stringify)
- [Crate on crates.io](https://crates.io/crates/enum-stringify)

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
