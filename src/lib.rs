//! Library returning the matching upper case letter A-Z (or a double letter AA-ZZ) for a given UTF-8 code point.
//!
//! # Exemple
//! ```rust
//! use to_uppercase_az::UppercaseAZ;
//!
//! let uppercase_az = UppercaseAZ::default();
//!
//! assert_eq!(uppercase_az['A'].to_string(), "A");
//! ```

mod expected;
pub use expected::{Letter, Property, Uppercase, UppercaseAZ, NOT_A_UPPERCASE};
