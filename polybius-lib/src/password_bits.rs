use rand::Rng;

use crate::user_data::Number;

/// A struct that represents a password bit, consisting of a two-character string and its importance to the user
pub struct PasswordBit {
    /// The two-character string representing a password bit
    pub bits: String,
    /// The importance of the password bit to the user (as a string)
    pub importance: String,
}

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
    ///
    /// # Arguments
    ///
    /// * `number` - The `Number` to convert to a `PasswordBit`
    ///
    /// # Returns
    ///
    /// A `PasswordBit` instance with the `bits` and `importance` fields set accordingly.
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
}
