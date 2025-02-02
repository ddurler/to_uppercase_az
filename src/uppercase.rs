/// DO NOT MODIFY
/// This file is automatically generated by command 'cargo run <filename>
///
use std::fmt::Display;

use crate::letter::Letter;
use crate::property::Property;

/// Unicode code point letter(s) and property
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uppercase {
    // Uppercase letter(s) equivalent
    pub letter: Letter,

    // Uppercase letter(s) properties
    pub property: Property,
}

impl Display for Uppercase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.letter)
    }
}

/// Default for not equivalent uppercase AZ
pub const NOT_AN_UPPERCASE: Uppercase = Uppercase {
    letter: Letter::Letter('?'),
    property: Property::NotAnUppercase,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_display_1() {
        let uppercase = Uppercase {
            letter: Letter::Letter('A'),
            property: Property::Capital,
        };
        assert_eq!(format!("{uppercase}"), "A");
    }

    #[test]
    fn test_uppercase_display_2() {
        let uppercase = Uppercase {
            letter: Letter::Letters('A', 'B'),
            property: Property::Capital,
        };
        assert_eq!(format!("{uppercase}"), "AB");
    }
}
