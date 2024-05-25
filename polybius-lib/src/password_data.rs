use std::fmt;

/// Represents the types of numbers that UserData can store. These types are used to specify the significance of the numbers to the user
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NumberType {
    BirthYear,
    BirthMonth,
    BirthDay,
    CurrentYear,
    RelevantNumber,
}

impl Default for NumberType {
    fn default() -> Self {
        NumberType::RelevantNumber
    }
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberType::BirthYear => write!(f, "Birth Year"),
            NumberType::BirthMonth => write!(f, "Birth Month"),
            NumberType::BirthDay => write!(f, "Birth Day"),
            NumberType::CurrentYear => write!(f, "Current Year"),
            NumberType::RelevantNumber => write!(f, "Relevant Number"),
        }
    }
}

/// Represents a numeric value along with its type.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Number {
    /// The numeric value
    pub value: u16,
    /// The type of the number
    pub num_type: NumberType,
}

impl Number {
    /// Creates a new Number instance with the given value and type.
    pub fn new(value: u16, num_type: NumberType) -> Self {
        Self { value, num_type }
    }
}

/// Contains all the information required for the password generation process.
pub struct PasswordData {
    /// A poll of positive numeric values and their importance to the user
    pub numbers_poll: Vec<Number>,
    /// A poll of strings of importance to the user
    pub text_poll: Vec<String>,
}

impl PasswordData {
    /// Creates a new UserData instance
    pub fn new(numbers_poll: Vec<Number>, text_poll: Vec<String>) -> Self {
        Self {
            numbers_poll,
            text_poll,
        }
    }
}

impl Default for PasswordData {
    /// Creates a new UserData instance with empty polls
    fn default() -> Self {
        Self {
            numbers_poll: Default::default(),
            text_poll: Default::default(),
        }
    }
}
