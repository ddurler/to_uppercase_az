//! Module to generate the rust source file for the hashmap containing the uppercase AZ code points

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

/// Rust source file to produce
const OUTPUT_RUST_FILE: &str = "./src/hash_uppercase_az.rs";

/// Structure for uppercase AZ code point
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EndPoint {
    /// UTF-8 code point
    code_point: u32,

    /// Textual description (Unicode database)
    description: String,

    /// Uppercase A-Z equivalent
    uppercase_az: String,

    /// true if uppercase letter
    is_capital: bool,

    /// true if lowercase letter
    is_small: bool,

    /// true if additional letter decoration (accent, cedilla, etc)
    is_decoration: bool,
}

impl EndPoint {
    pub const fn new(
        code_point: u32,
        description: String,
        uppercase_az: String,
        is_capital: bool,
        is_small: bool,
        is_decoration: bool,
    ) -> Self {
        Self {
            code_point,
            description,
            uppercase_az,
            is_capital,
            is_small,
            is_decoration,
        }
    }
}

/// Header of the rust source file
const HEADER: &str = r"// DO NOT MODIFY
// This file is automatically generated by command 'cargo run generate'
//
#[rustfmt::skip]
lazy_static! {
    static ref UPPERCASE_AZ: HashMap<u32, Uppercase> = {
        let mut m = HashMap::new();
";

/// Footer of the rust source file
const FOOTER: &str = r"
        m
    };
}
";

/// Generate the rust source file for the hashmap containing the uppercase AZ code points
pub fn generate_rust_file(endpoints: Vec<EndPoint>) {
    // Try to remove previous file
    match fs::remove_file(OUTPUT_RUST_FILE) {
        Ok(()) => println!("Previous version of '{OUTPUT_RUST_FILE}' has been deleted."),
        Err(_) => println!("A new file '{OUTPUT_RUST_FILE}' will be created."),
    }

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) //Create the file if it does not exist
        .open(OUTPUT_RUST_FILE)
        .expect("Unable to write the file");

    // File header
    write!(file, "{HEADER}").expect("Unable to write the file");

    // All endpoints
    for end_point in endpoints {
        let str_start = format!("        m.insert(0x{:X}", end_point.code_point);

        let str_begin = "Uppercase {";

        let vec_chars = end_point.uppercase_az.chars().collect::<Vec<char>>();
        let str_letter = match vec_chars.len() {
            1 => format!("letter: Letter::Letter('{}')", vec_chars[0]),
            2 => format!(
                "letter: Letter::Letters('{}', '{}')",
                vec_chars[0], vec_chars[1]
            ),
            _ => panic!(
                "Invalid uppercase_az '{}': 1 or 2 characters expected",
                end_point.uppercase_az
            ),
        };

        let str_property = match (
            end_point.is_capital,
            end_point.is_small,
            end_point.is_decoration,
        ) {
            (true, false, false) => "property: Property::Capital",
            (false, true, false) => "property: Property::Small",
            (true, true, false) => "property: Property::SmallAndCapital",
            (false, true, true) => "property: Property::SmallWithDecoration",
            (true, false, true) => "property: Property::CapitalWithDecoration",
            (true, true, true) => "property: Property::SmallAndCapitalWithDecoration",
            _ => "property: Property::Unknown",
        };

        let str_end = "});";

        let content = format!("{str_start}, {str_begin}{str_letter}, {str_property}{str_end}\n");
        write!(file, "{content}").expect("Unable to write the file");
    }

    // File footer
    write!(file, "{FOOTER}").expect("Unable to write the file");
}
