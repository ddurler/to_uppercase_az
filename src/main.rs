//! This crate uses the Unicode database file to identify code points that can be converted to uppercase A-Z.
//! Each line of this file (.csv) contains a code point (first column, in hex (text).
//!
//! The following definitions permit to filter Unicode end point description.
//! Once done, remaining description is the uppercase A-Z equivalent.
//! Only one or two uppercase A-Z are identified (three letters are ignored)

use std::fs::File;
use std::io::{BufRead, BufReader};

mod expected;

/// Unicode database data file
const UNICODE_DATA_FILE: &str = "./unicode_database/UnicodeData.txt";

/// The description must contain the following keywords to be considered as a LATIN LETTER:
const UNICODE_LATIN: [&str; 1] = ["LATIN"];

/// The description must contain one of the following keywords to be considered as a LATIN LETTER:
const UNICODE_LATIN_LETTERS: [&str; 2] = ["LETTER", "LIGATURE"];

/// The description must not contain one of the following keywords be considered as a LATIN LETTER:
const UNICODE_NOT_LATIN_LETTERS: [&str; 70] = [
    "ALPHA",
    "ALVEOLAR",
    "BASELINE ESH",
    "BASELINE ETH",
    "BETA",
    "BIDENTAL",
    "BILABIAL",
    "CAPITAL ETH",
    "CAPITAL EZH",
    "CAPITAL RUM",
    "CHI",
    "CUATRILLO",
    "LAMBDA",
    "DELTA",
    "DENTAL",
    "DESH",
    "DEZH",
    "EGYPTOLOGICAL",
    "GAMMA",
    "HENG",
    "HWAIR",
    "INPUT SYMBOL",
    "IOTA",
    "LATERAL",
    "LATINATE",
    "LETTER AIN",
    "LETTER CON",
    "LETTER DUM",
    "LETTER ENG",
    "LETTER ESH",
    "LETTER ETH",
    "LETTER EZH",
    "LETTER FENG",
    "LETTER FFI",
    "LETTER FFL",
    "LETTER KRA",
    "LETTER LUM",
    "LETTER MUM",
    "LETTER NUM",
    "LETTER RUM",
    "LETTER RETROFLEX",
    "LETTER REVERSED ESH",
    "LETTER TWO",
    "LETTER TUM",
    "LEZH",
    "LIGATURE FFI",
    "LIGATURE FFL",
    "OMEGA",
    "PHI",
    "RAMS HORN",
    "REVERSED ENG",
    "REVERSED ESH",
    "RUM ROTUNDA",
    "SAKHA YAT",
    "SALTILLO",
    "SINOLOGICAL",
    "STOP",
    "SCHWA",
    "TESH",
    "THORN",
    "TONE TWO",
    "TONE FIVE",
    "TONE SIX",
    "TRESILLO",
    "UPSILON",
    "VEND",
    "VOICED",
    "WYNN",
    "YOGH",
    "YUS",
];

/// List of decoration keywords for latin letters description
const DECORATION_KEYWORDS: [&str; 113] = [
    "ABOVE",
    "ACUTE",
    "AFRICAN",
    "ANGLICANA",
    "ARCHAIC",
    "BARRED",
    "BELOW",
    "BELT",
    "BLACK",
    "BOTTOM",
    "BRACKETED",
    "BREVE",
    "BROKEN",
    "COMMA",
    "CARON",
    "CEDILLA",
    "CIRCLED",
    "CIRCUMFLEX",
    "CLOSED",
    "COMBINING",
    "CROSSED-TAIL",
    "CURL",
    "DESCENDER",
    "DIAGONAL",
    "DIAERESIZED",
    "DIAERESIS",
    "DIGRAPH",
    "DOUBLE",
    "DOTLESS",
    "EPIGRAPHIC",
    "FISH",
    "FLATTENED",
    "FLOURISH",
    "FULLWIDTH",
    "GLOTTAL",
    "GRAVE",
    "HALF",
    "HANDLE",
    "HIGH STROKE",
    "HOOK",
    "HORIZONTAL",
    "HORN",
    "INSULAR",
    "INSIDE",
    "IOTIFIED",
    "ITALIC",
    "INVERTED",
    "LIGHT CENTRALIZATION",
    "LAZY S",
    "LEFT",
    "LEG",
    "LENIS",
    "LONGA",
    "LONG",
    "LOOP",
    "LOW",
    "MACRON",
    "MID-HEIGHT",
    "MIDDLE-WELSH",
    "MIDDLE",
    "NEGATIVE",
    "NOTCH",
    "OBLIQUE",
    "OGONEK",
    "OPEN-O",
    "OPEN",
    "OUTLINED",
    "OVERLAY",
    "PARENTHESIZED",
    "PALATAL",
    "PRECEDED BY APOSTROPHE",
    "POLISH",
    "RETROFLEX",
    "REVERSED-SCHWA",
    "REVERSED",
    "RIGHT",
    "RING",
    "ROTUNDA",
    "SHARP",
    "SCOTS",
    "SERIF",
    "SHELL",
    "SHORT",
    "SIDEWAYS",
    "SIGMOID",
    "SQUARED ",
    "SQUIRREL TAIL",
    "STIRRUP",
    "STRETCHED",
    "STRIKETHROUGH",
    "STROKE",
    "SUBSCRIPT",
    "SUPERSCRIPT",
    "SWASH TAIL",
    "THROUGH",
    "TILDE",
    "TOPBAR",
    "TORTOISE",
    "TURNED",
    "VISIGOTHIC",
    "VOLAPUK",
    // Keep the following keywords at the end of the list
    // as early remove from description may alter the filtering
    "AND",
    "BAR",
    "CROSSED",
    "DOT",
    "LINE",
    "OLD",
    "SCRIPT",
    "TAG",
    "TAIL",
    "TOP",
    "WITHOUT",
    "WITH",
];

/// Structure for uppercase AZ code point
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UppercaseAZ {
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
}

/// Parse the Unicode database file and identify endpoints that can be converted to uppercase A-Z
fn parse_unicode_database_file(filename: &str) -> Result<Vec<UppercaseAZ>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut nb_lines = 0;
    let mut used_decoration_keywords = vec![];

    let mut all_uppercase_az = vec![];

    for line in reader.lines() {
        nb_lines += 1;
        let line = line?;
        if let Some((code_point, description)) = is_latin_letter(nb_lines, &line) {
            let (uppercase_az, is_capital, is_small, is_decoration, local_used_keyword) =
                parse_latin_letter(&description);
            used_decoration_keywords.extend(local_used_keyword);

            // println!("{code_point:04X} '{uppercase_az}' (capital={is_capital}, small={is_small}, decoration={is_decoration}) from '{description}'");
            all_uppercase_az.push(UppercaseAZ {
                code_point,
                description,
                uppercase_az,
                is_capital,
                is_small,
                is_decoration,
            });
        }
    }

    // Checks for unused descriptions
    for keyword in DECORATION_KEYWORDS {
        if !used_decoration_keywords.contains(&keyword.to_string()) {
            eprintln!("!!! Unused decoration keyword: '{keyword}'");
        }
    }

    Ok(all_uppercase_az)
}

/// Returns the code point and the description if the current Unicode line in database is a latin letter
fn is_latin_letter(line_nb: usize, line_content: &str) -> Option<(u32, String)> {
    // Line is .csv file
    let elements: Vec<String> = line_content
        .split(';')
        .map(|s| s.trim().to_string())
        .take(2)
        .collect();
    if elements.len() < 2 {
        println!("Invalid line #{line_nb}: {line_content}");
        return None;
    }

    // First element contains the code point in hex
    let code_point = match u32::from_str_radix(&elements[0], 16) {
        Ok(code_point) => code_point,
        Err(err) => {
            println!(
                "Invalid code_point {} on line #{line_nb} ({err})",
                elements[0]
            );
            return None;
        }
    };

    // Second element contains the textual description of the code point
    if is_valid_latin_letter(&elements[1]) {
        let name = elements[1].clone();
        Some((code_point, name))
    } else {
        None
    }
}

/// Returns true if the description matches a valid latin letter
fn is_valid_latin_letter(description: &str) -> bool {
    // Mandatory keywords
    for keyword in UNICODE_LATIN {
        if !description.contains(keyword) {
            return false;
        }
    }

    // One of optional keywords
    let mut is_latin_letter = false;
    for keyword in UNICODE_LATIN_LETTERS {
        if description.contains(keyword) {
            is_latin_letter = true;
            break;
        }
    }
    if !is_latin_letter {
        return false;
    }

    // Forbidden keywords
    for keyword in UNICODE_NOT_LATIN_LETTERS {
        let keyword_with_space_before = format!(" {keyword}");
        let keyword_with_space_after = format!("{keyword} ");
        if description.contains(&keyword_with_space_before)
            || description.contains(&keyword_with_space_after)
        {
            return false;
        }
    }

    true
}

/// Parse the LATIN LETTER description to get matching uppercase letter A-Z and some properties.
///
/// 3 properties are extracted:
/// * `is_capital` - true if the letter is uppercase
/// * `is_small` - true if the letter is lowercase
/// * `is_decoration` - true if the letter has some extra graphical decoration
///
/// Note that a letter can be both capital and small: A small capital letter is also a letter.
fn parse_latin_letter(description: &str) -> (String, bool, bool, bool, Vec<String>) {
    // Local function to check if the name contains a keyword and remove this keyword
    fn contains_keyword(description: &mut String, keyword: &str) -> bool {
        let keyword_with_space_before = format!(" {keyword}");
        let keyword_with_space_after = format!("{keyword} ");
        if description.contains(&keyword_with_space_before)
            || description.contains(&keyword_with_space_after)
        {
            *description = description.replace(keyword, "");
            true
        } else {
            false
        }
    }

    let initial_description = description;

    // Remove "LATIN ", "LETTER" and "LIGATURE" from description
    let mut description = description
        .replace("LATIN", "")
        .replace("LETTER", "")
        .replace("LIGATURE", "");

    // Check for CAPITAL and SMALL and remove if found
    let is_capital = contains_keyword(&mut description, "CAPITAL");
    let is_small = contains_keyword(&mut description, "SMALL");

    // Check for DECORATION
    let mut used_decoration_keywords = Vec::new();
    let mut is_decoration = false;

    for keyword in DECORATION_KEYWORDS {
        if contains_keyword(&mut description, keyword) {
            is_decoration = true;
            used_decoration_keywords.push(keyword.to_string());
        }
    }

    // Get final letters
    let uppercase_az: String = description
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    if uppercase_az.is_empty() {
        println!("!!! NO DESCRIPTION !!! (capital={is_capital}, small={is_small}, decoration={is_decoration}) from '{initial_description}'");
    }
    if uppercase_az.len() > 2 {
        println!("!!! '{uppercase_az}' (capital={is_capital}, small={is_small}, decoration={is_decoration}) from '{initial_description}'");
    }

    (
        uppercase_az,
        is_capital,
        is_small,
        is_decoration,
        used_decoration_keywords,
    )
}
