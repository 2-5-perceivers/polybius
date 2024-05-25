use rand::Rng;

use crate::password_data::Number;

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
            bits: Self::number_to_bit(&number.value),
            importance: number.num_type.to_string(),
        }
    }

    fn number_to_bit(number: &u16) -> String {
        // If the number is longer that 2 digits, we only take the last 2 digits. Ensure that if the number is less the 10 we add a 0 in front of it.
        let number = number % 100;
        if number < 10 {
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
