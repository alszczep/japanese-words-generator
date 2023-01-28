use std::{fs::File, io::{Write, LineWriter}};
use rand::Rng;

use crate::{kana::{Syllabary}, config::{Config}};

struct TranslatedStrings {
    pub kana: String,
    pub romaji: String,
}

pub fn generate_files(_syllabary: Syllabary, cfg: Config) -> std::io::Result<()> {
    let syllabary = filter_syllabary(_syllabary, cfg.clone().kana_types);

    let mut kana_writer = LineWriter::new(File::create("./kana.txt")?);
    let mut romaji_writer = LineWriter::new(File::create("./romaji.txt")?);
    
    for _ in 0..cfg.clone().lines_count {
        let line = generate_line(syllabary.clone(), cfg.clone());
        kana_writer.write_all(line.kana.as_bytes())?;
        kana_writer.write_all(b"\n")?;
        romaji_writer.write_all(line.romaji.as_bytes())?;
        romaji_writer.write_all(b"\n")?;
    }

    Ok(())
}

fn filter_syllabary(mut syllabary: Syllabary, kana_types: Vec<String>) -> Syllabary {
    syllabary.drain_filter(|kana| kana_types.contains(&kana.kana_type)).collect()
}

fn generate_line(syllabary: Syllabary, cfg: Config) -> TranslatedStrings {
    let mut kana_line = "".to_string();
    let mut romaji_line = "".to_string();

    for _ in 0..cfg.words_per_line {
        for character in generate_word(syllabary.clone(), cfg.clone()) {
            kana_line.push_str(&character.kana);
            romaji_line.push_str(&character.romaji);
        }

        if !cfg.clone().no_spaces_in_kana {
            kana_line.push_str(" ");
        }
        romaji_line.push_str(" ");
    }

    TranslatedStrings { kana: kana_line, romaji: romaji_line }
}

fn generate_word(syllabary: Syllabary, cfg: Config) -> Vec<TranslatedStrings> {
    if syllabary.len() == 0 {
        return vec![];
    }

    let mut rng = rand::thread_rng();

    let mut word_length = 1;

    let random_chance = rng.gen_range(0..99);
    let mut current_chance_checked = 0;
    for word_chance in cfg.word_chances {
        if random_chance >= current_chance_checked && random_chance < current_chance_checked + word_chance.chance {
            word_length = word_chance.length;
            break;
        }

        current_chance_checked += word_chance.chance;
    }
    let mut translated: Vec<TranslatedStrings> = vec![];
    for _ in 0..word_length {
        let character_index = rng.gen_range(0..(syllabary.len() - 1));
        let character = &syllabary[character_index];
        translated.push(TranslatedStrings { kana: character.clone().kana, romaji: character.clone().romaji });
    }

    translated
}
