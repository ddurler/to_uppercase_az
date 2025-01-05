//! Library returning the matching upper case letter A-Z (or a double letter AA-ZZ) for a given UTF-8 code point.
//!
//! # Exemple
//! ```rust
//! use to_uppercase_az::UppercaseAZ;
//!
//! let uppercase_az = UppercaseAZ::default();
//!
//! assert_eq!(uppercase_az['A'].to_string(), "A");
//! assert_eq!(uppercase_az.to_string("A2B"), "A2B");
//! ```

mod letter;
pub use letter::Letter;

mod property;
pub use property::Property;

mod uppercase;
pub use uppercase::{Uppercase, NOT_A_UPPERCASE};

mod uppercase_az;
pub use uppercase_az::UppercaseAZ;
