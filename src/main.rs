use crate::kana::Syllabary;

pub mod kana;
pub mod config;
pub mod input;

fn main() {
    let cfg = config::load_config();
    let syllabary_input = input::ask_for_input();
    let mut syllabary: Syllabary;
    match syllabary_input {
        input::Input::Hirgana => syllabary = kana::load_hiragana(),
        input::Input::Katakana => syllabary = kana::load_katakana(),
    }
}
