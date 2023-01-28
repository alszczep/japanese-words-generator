use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Syllabary = Vec<Kana>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kana {
    pub kana: String,
    pub romaji: String,
    pub kana_type: String,
}

pub fn load_hiragana() -> Syllabary {
    let string_json = include_str!("../data/hiragana.json");

    serde_json::from_str(string_json)
        .expect(format!("Unable to parse hiragana.json").as_str())
}

pub fn load_katakana() -> Syllabary {
    let string_json = include_str!("../data/katakana.json");

    serde_json::from_str(string_json)
        .expect(format!("Unable to parse katakana.json").as_str())
}
