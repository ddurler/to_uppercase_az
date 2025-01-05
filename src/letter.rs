//! Structure for one or two letters equivalent for uppercase AZ code point
use std::fmt::Display;

/// Enumeration for uppercase AZ letter equivalent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    /// Single uppercase letter equivalent
    Letter(char),

    /// Double uppercase letter equivalent
    Letters(char, char),
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Letter(c) => write!(f, "{c}"),
            Self::Letters(c1, c2) => write!(f, "{c1}{c2}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_1_letter() {
        let letter = Letter::Letter('A');
        assert_eq!(letter.to_string(), "A");
    }

    #[test]
    fn test_display_2_letters() {
        let letter = Letter::Letters('A', 'B');
        assert_eq!(letter.to_string(), "AB");
    }
}
