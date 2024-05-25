use rand::Rng;

use crate::password_data::{Number, NumberType};

/// A struct that represents a password bit, consisting of a two-character string and its importance to the user
pub struct PasswordBit {
    /// The two-character string representing a password bit
    pub bits: String,
    /// The importance of the password bit to the user (as a string)
    pub importance: String,
}

pub type PasswordBits = Vec<PasswordBit>;

impl PasswordBit {
    /// Create a new PasswordBits instance
    pub fn new(bits: String, importance: String) -> Self {
        PasswordBit { bits, importance }
    }

    /// Create a new PasswordBit instance from a Number
    ///
    /// This function takes a `Number` as input and returns a `PasswordBit` instance
    /// with the `bits` field set to the last two digits of the number and the `importance`
    /// field set to the string representation of the number's `num_type`.
    pub fn number_bit(number: &Number) -> PasswordBit {
        PasswordBit {
            bits: Self::number_to_bit(&number.value, &number.num_type),
            importance: number.num_type.to_string(),
        }
    }

    fn number_to_bit(number: &u16, number_type: &NumberType) -> String {
        // If the number type is BirthMonth or BirthDay, we only want two digits with a leading 0 if the case.
        // If the number type is BirthYear or CurrentYear, we want to randomly choose between the full year or the last two digits.
        // If the number type is RelevantNumber, we want the full number.
        match number_type {
            NumberType::BirthMonth | NumberType::BirthDay => Self::truncate_number(number),
            NumberType::BirthYear | NumberType::CurrentYear => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    number.to_string()
                } else {
                    Self::truncate_number(number)
                }
            }
            NumberType::RelevantNumber => number.to_string(),
        }
    }

    /// Truncate a number to a two digits string. If less then 10, add a leading 0.
    fn truncate_number(number: &u16) -> String {
        let number = number % 100;
        if number == 0 {
            return "00".to_string();
        } else if number < 10 {
            format!("0{}", number)
        } else {
            number.to_string()
        }
    }

    /// Create a new PasswordBit instance from a String. It randomly selects one to three characters from the beginning of the string.
    pub fn string_bit(string: &str) -> PasswordBit {
        let mut rng = rand::thread_rng();
        let bits = string.chars().take(rng.gen_range(1..4)).collect();
        PasswordBit {
            bits,
            importance: string.to_string(),
        }
    }

    /// Create a new PasswordBit instance from a symbol. It randomly selects one of the symbols from the list.
    /// The list is !, @, #, $, %, ^, &, *, (, ), -, _, +, =
    pub fn symbol_bit() -> PasswordBit {
        let mut rng = rand::thread_rng();

        // The default symbols to choose from. Based on the International Keyboard Layout top row. To be easily typed by the user.
        let symbols = vec![
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=',
        ];
        let bits = symbols[rng.gen_range(0..symbols.len())];
        PasswordBit {
            bits: bits.to_string(),
            importance: "Symbol".to_string(),
        }
    }
}
