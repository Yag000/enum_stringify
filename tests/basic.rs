#[derive(enum_string::EnumToString)]
enum Numbers {
    One,
    Two,
    Three,
}

#[test]
fn test_numbers() {
    assert_eq!(Numbers::One.to_string(), "One");
    assert_eq!(Numbers::Two.to_string(), "Two");
    assert_eq!(Numbers::Three.to_string(), "Three");
}

#[derive(enum_string::EnumToString, Debug, PartialEq)]
#[allow(non_camel_case_types)]
enum Command {
    /// Ignore this comment macro :)
    HelpMe,
    /// And this one too!
    yay,
    /// This is a help Command
    _wow,
}

#[test]
fn test_command() {
    assert_eq!(Command::HelpMe.to_string(), "HelpMe");
    assert_eq!(Command::yay.to_string(), "yay");
    assert_eq!(Command::_wow.to_string(), "_wow");
}
