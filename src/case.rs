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
