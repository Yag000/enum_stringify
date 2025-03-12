use convert_case::Casing;

pub(crate) struct Case(convert_case::Case);

impl TryFrom<(String, String)> for Case {
    type Error = ();

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        if value.0 == "case" {
            value.1.try_into()
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for Case {
    type Error = ();

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
            _ => Err(())?,
        }))
    }
}

impl Case {
    pub(crate) fn to_case(&self, s: &str) -> String {
        s.to_case(self.0)
    }
}
