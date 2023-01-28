#![feature(drain_filter)]
pub mod kana;
pub mod config;
pub mod input;
pub mod generator;

fn main() {
    let cfg = config::load_config();

    let syllabary: kana::Syllabary;
    match input::ask_for_input() {
        input::Input::Hirgana => syllabary = kana::load_hiragana(),
        input::Input::Katakana => syllabary = kana::load_katakana(),
    }

    generator::generate_files(syllabary, cfg).expect("Failed to generate files");

    println!("Generated!");
}
