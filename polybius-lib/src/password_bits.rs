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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_password_bit() {
        let password_bit = PasswordBit::new("ab".to_string(), "abc".to_string());
        assert_eq!(password_bit.bits, "ab");
        assert_eq!(password_bit.importance, "abc");
    }

    #[test]
    fn test_number_bit() {
        let number_1 = Number {
            value: 123,
            num_type: NumberType::BirthDay,
        };
        let number_2 = Number {
            value: 456,
            num_type: NumberType::BirthMonth,
        };
        let number_3 = Number {
            value: 1999,
            num_type: NumberType::BirthYear,
        };
        let number_4 = Number {
            value: 2024,
            num_type: NumberType::CurrentYear,
        };
        let number_5 = Number {
            value: 12345,
            num_type: NumberType::RelevantNumber,
        };

        let password_bit_1 = PasswordBit::number_bit(&number_1);
        let password_bit_2 = PasswordBit::number_bit(&number_2);
        let password_bit_3 = PasswordBit::number_bit(&number_3);
        let password_bit_4 = PasswordBit::number_bit(&number_4);
        let password_bit_5 = PasswordBit::number_bit(&number_5);

        assert_eq!(
            password_bit_1.bits, "23",
            "Expected: 23, got: {}",
            password_bit_1.bits
        );
        assert_eq!(
            password_bit_1.importance, "Birth Day",
            "Expected: Birth Day, got: {}",
            password_bit_1.importance
        );

        assert_eq!(
            password_bit_2.bits, "56",
            "Expected: 56, got: {}",
            password_bit_2.bits
        );
        assert_eq!(
            password_bit_2.importance, "Birth Month",
            "Expected: Birth Month, got: {}",
            password_bit_2.importance
        );

        assert!(
            password_bit_3.bits == "99" || password_bit_3.bits == "1999",
            "Expected: 99 or 1999, got: {}",
            password_bit_3.bits
        );
        assert_eq!(
            password_bit_3.importance, "Birth Year",
            "Expected: Birth Year, got: {}",
            password_bit_3.importance
        );

        assert!(
            password_bit_4.bits == "24" || password_bit_4.bits == "2024",
            "Expected: 24 or 2024, got: {}",
            password_bit_4.bits
        );
        assert_eq!(
            password_bit_4.importance, "Current Year",
            "Expected: Current Year, got: {}",
            password_bit_4.importance
        );

        assert_eq!(
            password_bit_5.bits, "12345",
            "Expected: 12345, got: {}",
            password_bit_5.bits
        );
        assert_eq!(
            password_bit_5.importance, "Relevant Number",
            "Expected: Relevant Number, got: {}",
            password_bit_5.importance
        );
    }

    #[test]
    fn test_truncate_number() {
        assert_eq!(PasswordBit::truncate_number(&0), "00");
        assert_eq!(PasswordBit::truncate_number(&1), "01");
        assert_eq!(PasswordBit::truncate_number(&9), "09");
        assert_eq!(PasswordBit::truncate_number(&10), "10");
        assert_eq!(PasswordBit::truncate_number(&99), "99");
        assert_eq!(PasswordBit::truncate_number(&100), "00");
        assert_eq!(PasswordBit::truncate_number(&2000), "00");
    }

    #[test]
    fn test_string_bit() {
        let password_bit = PasswordBit::string_bit("hello");
        assert!(
            password_bit.bits.len() >= 1 && password_bit.bits.len() <= 3,
            "Expected password bits to be between 1 and 3 characters, got: {}",
            password_bit.bits
        );
        assert!(
            "hello".starts_with(&password_bit.bits),
            "Expected passwords bits to be h/he/hel, got: {}",
            password_bit.bits
        );
        assert_eq!(password_bit.importance, "hello");
    }

    #[test]
    fn test_symbol_bit() {
        let password_bit = PasswordBit::symbol_bit();
        let symbols = vec![
            "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "+", "=",
        ];
        assert!(symbols.contains(&password_bit.bits.as_str()));
        assert_eq!(password_bit.importance, "Symbol");
    }
}
