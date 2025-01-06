//!
//! This tool uses then [Unicode database file](https://www.unicode.org/Public/UNIDATA/UnicodeData.txt) to
//! identify `UTF_8` endpoints that can be converted to uppercase A-Z.
//!
//! ```cmd
//! $ cargo run
//! ```
//!
//! When used with the `generate` argument, this tool will generate the rust source file for the
//! uppercase AZ equivalent.
//!
//! ```cmd
//! $ cargo run generate```
//!

mod letter;
mod property;
mod uppercase;
mod uppercase_az;

mod database_parsing;
use database_parsing::parse_unicode_database_file;

mod hash_generation;
use hash_generation::generate_rust_file;

/// Unicode database data file
const UNICODE_DATA_FILE: &str = "./unicode_database/UnicodeData.txt";

fn main() {
    println!("Unicode database file parser now running on file {UNICODE_DATA_FILE}...");
    let all_uppercase_az = match parse_unicode_database_file(UNICODE_DATA_FILE) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {UNICODE_DATA_FILE}: {err}");
            std::process::exit(1);
        }
    };
    println!(
        "Found {} code points with A-Z equivalent",
        all_uppercase_az.len()
    );
    println!("Unicode database file parsing done.");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "generate" {
        generate_rust_file(all_uppercase_az);
    } else {
        println!("Usage: 'cargo run generate' to generate the rust source file for this crate.");
    }
}
