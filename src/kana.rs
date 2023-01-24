use std::fs;
use serde_derive::Deserialize;
use serde_derive::Serialize;

const HIRAGANA_FILE: &str = "hiragana.json";
const KATAKANA_FILE: &str = "katakana.json";

pub type Syllabary = Vec<Kana>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kana {
    pub kana: String,
    pub romaji: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub fn load_syllabary(filename: &str) -> Syllabary {
    let string_json: String = 
        fs::read_to_string(format!("src/{file}", file = filename))
        .expect(format!("Unable to read {file}", file = filename).as_str());

    let parsed_json: Syllabary = 
        serde_json::from_str(string_json.as_str())
        .expect(format!("Unable to parse {file}", file = filename).as_str());

    parsed_json
}

pub fn load_hiragana() -> Syllabary {
    load_syllabary(HIRAGANA_FILE)
}

pub fn load_katakana() -> Syllabary {
    load_syllabary(KATAKANA_FILE)
}
