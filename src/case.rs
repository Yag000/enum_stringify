use std::fmt::Display;

use convert_case::Casing;

/// Wrapper struct around `convert_case::Case` to represent different casing styles.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Case(convert_case::Case);

// This is used to check if the first string is "case" and then attempt conversion of the second string.
impl TryFrom<(String, String)> for Case {
    type Error = &'static str;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "case" {
            value.1.try_into()
        } else {
            Err("The first string is not \"case\"")
        }
    }
}

// Maps specific string values to their corresponding `convert_case::Case` variant.
impl TryFrom<String> for Case {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(match value.as_str() {
            "\"upper\"" => convert_case::Case::Upper,
            "\"lower\"" => convert_case::Case::Lower,
            "\"title\"" => convert_case::Case::Title,
            "\"toggle\"" => convert_case::Case::Toggle,
            "\"camel\"" => convert_case::Case::Camel,
            "\"pascal\"" => convert_case::Case::Pascal,
            "\"upper_camel\"" => convert_case::Case::UpperCamel,
            "\"snake\"" => convert_case::Case::Snake,
            "\"upper_snake\"" => convert_case::Case::UpperSnake,
            "\"screaming_snake\"" => convert_case::Case::ScreamingSnake,
            "\"kebab\"" => convert_case::Case::Kebab,
            "\"cobol\"" => convert_case::Case::Cobol,
            "\"upper_kebab\"" => convert_case::Case::UpperKebab,
            "\"train\"" => convert_case::Case::Train,
            "\"flat\"" => convert_case::Case::Flat,
            "\"upper_flat\"" => convert_case::Case::UpperFlat,
            "\"alternating\"" => convert_case::Case::Alternating,
            _ => Err("Invalid case")?,
        }))
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let v = match self.0 {
            convert_case::Case::Upper => "upper".to_string(),
            convert_case::Case::Lower => "lower".to_string(),
            convert_case::Case::Title => "title".to_string(),
            convert_case::Case::Toggle => "toggle".to_string(),
            convert_case::Case::Camel => "camel".to_string(),
            convert_case::Case::Pascal => "pascal".to_string(),
            convert_case::Case::UpperCamel => "upper_camel".to_string(),
            convert_case::Case::Snake => "snake".to_string(),
            convert_case::Case::UpperSnake => "upper_snake".to_string(),
            convert_case::Case::ScreamingSnake => "screaming_snake".to_string(),
            convert_case::Case::Kebab => "kebab".to_string(),
            convert_case::Case::Cobol => "cobol".to_string(),
            convert_case::Case::UpperKebab => "upper_kebab".to_string(),
            convert_case::Case::Train => "train".to_string(),
            convert_case::Case::Flat => "flat".to_string(),
            convert_case::Case::UpperFlat => "upper_flat".to_string(),
            convert_case::Case::Alternating => "alternating".to_string(),
        };
        write!(f, "{v}")
    }
}

impl Case {
    /// Applies the stored casing style to the given string `s` and returns the formatted result.
    pub(crate) fn to_case(&self, s: &str) -> String {
        s.to_case(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use convert_case::Case as CC;

    /// Helper to wrap a convert_case::Case into our Case type.
    fn wrap(c: CC) -> Case {
        Case(c)
    }

    /// List of all supported case strings and their expected enum variants.
    fn all_cases() -> Vec<(&'static str, CC)> {
        vec![
            ("\"upper\"", CC::Upper),
            ("\"lower\"", CC::Lower),
            ("\"title\"", CC::Title),
            ("\"toggle\"", CC::Toggle),
            ("\"camel\"", CC::Camel),
            ("\"pascal\"", CC::Pascal),
            ("\"upper_camel\"", CC::UpperCamel),
            ("\"snake\"", CC::Snake),
            ("\"upper_snake\"", CC::UpperSnake),
            ("\"screaming_snake\"", CC::ScreamingSnake),
            ("\"kebab\"", CC::Kebab),
            ("\"cobol\"", CC::Cobol),
            ("\"upper_kebab\"", CC::UpperKebab),
            ("\"train\"", CC::Train),
            ("\"flat\"", CC::Flat),
            ("\"upper_flat\"", CC::UpperFlat),
            ("\"alternating\"", CC::Alternating),
        ]
    }

    // ------------------------------------------------------------------------
    // Parsing Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_try_from_string_parses_all_cases() {
        for (input, expected_variant) in all_cases() {
            let parsed: Case = input.to_string().try_into().unwrap();
            assert_eq!(
                parsed,
                wrap(expected_variant),
                "Parsing string {input} should yield {expected_variant:?}"
            );
        }
    }

    #[test]
    fn test_try_from_string_rejects_invalid_input() {
        let err = Case::try_from("invalid_value".to_string());
        assert!(err.is_err());
    }

    #[test]
    fn test_try_from_tuple_parses_when_first_value_is_case() {
        let (key, val) = ("case".to_string(), "\"upper\"".to_string());
        let result: Result<Case, _> = (key, val).try_into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_from_tuple_rejects_when_first_value_is_not_case() {
        let (key, val) = ("not_case".to_string(), "\"upper\"".to_string());
        let result: Result<Case, _> = (key, val).try_into();
        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Display Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_display_outputs_all_expected_strings() {
        for (input_literal, variant) in all_cases() {
            let c = wrap(variant);
            let expected_display = input_literal.trim_matches('"');
            assert_eq!(
                c.to_string(),
                expected_display,
                "Display for {:?} should be `{}`",
                variant,
                expected_display
            );
        }
    }

    // ------------------------------------------------------------------------
    // to_case() Behavior Documentation Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_to_case_produces_expected_conversions() {
        let example = "hello world";

        let examples = vec![
            (CC::Upper, "HELLO WORLD"),
            (CC::Lower, "hello world"),
            (CC::Title, "Hello World"),
            (CC::Toggle, "hELLO wORLD"),
            (CC::Camel, "helloWorld"),
            (CC::Pascal, "HelloWorld"),
            (CC::UpperCamel, "HelloWorld"),
            (CC::Snake, "hello_world"),
            (CC::UpperSnake, "HELLO_WORLD"),
            (CC::ScreamingSnake, "HELLO_WORLD"),
            (CC::Kebab, "hello-world"),
            (CC::Cobol, "HELLO-WORLD"),
            (CC::UpperKebab, "HELLO-WORLD"),
            (CC::Train, "Hello-World"),
            (CC::Flat, "helloworld"),
            (CC::UpperFlat, "HELLOWORLD"),
            (CC::Alternating, "hElLo WoRlD"),
        ];

        for (variant, expected_output) in examples {
            let c = Case(variant);
            assert_eq!(
                c.to_case(example),
                expected_output,
                "Case {:?} should convert `{}` to `{}`",
                variant,
                example,
                expected_output
            );
        }
    }
}
