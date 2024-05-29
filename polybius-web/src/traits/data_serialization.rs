use polybius_lib::password_data::NumberType;

// Quick and dirty serialization/deserialization trait for data that goes into a select element
pub trait DataSerialization {
    fn from_string(s: &str) -> Self;
    fn into_string(self) -> &'static str;
}

impl DataSerialization for NumberType {
    fn from_string(s: &str) -> Self {
        match s {
            "birth_year" => NumberType::BirthYear,
            "birth_month" => NumberType::BirthMonth,
            "birth_day" => NumberType::BirthDay,
            "current_year" => NumberType::CurrentYear,
            _ => NumberType::RelevantNumber,
        }
    }

    fn into_string(self) -> &'static str {
        match self {
            NumberType::BirthYear => "birth_year",
            NumberType::BirthMonth => "birth_month",
            NumberType::BirthDay => "birth_day",
            NumberType::CurrentYear => "current_year",
            NumberType::RelevantNumber => "relevant_number",
        }
    }
}
