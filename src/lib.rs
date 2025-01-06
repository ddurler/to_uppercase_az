//! Library returning the matching upper case letter A-Z (or a double letter AA-ZZ) for a given UTF-8 code point.
//!
//! The uppercase AZ equivalent is based on the [Unicode database file](https://www.unicode.org/Public/UNIDATA/UnicodeData.txt).
//!
//! # Examples
//!
//! Basic usage for uppercase AZ equivalent :
//!
//! ```rust
//! use to_uppercase_az::UppercaseAZ;
//!
//! let uppercase_az = UppercaseAZ::default();
//!
//! assert_eq!(uppercase_az['A'].to_string(), "A");
//! assert_eq!(uppercase_az.to_string("à l'œil"), "A L'OEIL");
//! ```
//!
//! Extra-information [`Letter`] and [`Property`] also available for uppercase AZ equivalent :
//!
//! ```rust
//! use to_uppercase_az::{UppercaseAZ, Letter, Property, NOT_AN_UPPERCASE};
//!
//! let uppercase_az = UppercaseAZ::default();
//!
//! assert_eq!(uppercase_az['A'].letter, Letter::Letter('A'));
//! assert_eq!(uppercase_az['A'].property, Property::Capital);
//! assert_eq!(uppercase_az['A'].property.is_capital(), true);
//! assert_eq!(uppercase_az['A'].property.is_small(), false);
//! assert_eq!(uppercase_az['A'].property.is_decoration(), false);
//!
//! assert_eq!(uppercase_az['2'], NOT_AN_UPPERCASE);
//! assert_eq!(uppercase_az['2'].property, Property::NotAnUppercase);
//!
//! assert_eq!(uppercase_az['a'].letter, Letter::Letter('A'));
//! assert_eq!(uppercase_az['a'].property, Property::Small);
//!
//! assert_eq!(uppercase_az['À'].letter, Letter::Letter('A'));
//! assert_eq!(uppercase_az['À'].property, Property::CapitalWithDecoration);
//!
//! assert_eq!(uppercase_az['Æ'].letter, Letter::Letters('A', 'E'));
//! assert_eq!(uppercase_az['Æ'].property, Property::Capital);
//!
//! assert_eq!(uppercase_az['æ'].letter, Letter::Letters('A', 'E'));
//! assert_eq!(uppercase_az['æ'].property, Property::Small);
//!
//! assert_eq!(uppercase_az['ǅ'].letter, Letter::Letters('D', 'Z'));
//! assert_eq!(uppercase_az['ǅ'].property, Property::SmallAndCapitalWithDecoration);
//! ```

mod letter;
pub use letter::Letter;

mod property;
pub use property::Property;

mod uppercase;
pub use uppercase::{Uppercase, NOT_AN_UPPERCASE};

mod uppercase_az;
pub use uppercase_az::UppercaseAZ;
