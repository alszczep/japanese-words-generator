use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct WordChance {
    pub chance: u8,
    pub lenght: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub kana_types: Vec<String>,
    pub words_count: u32,
    pub words_per_line: u32,
    pub word_chances: Vec<WordChance>,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { 
        kana_types: vec!["gojūon".to_string(), "dakuten".to_string(), "handakuten".to_string(), "yōon".to_string()], 
        words_count: 50,
        words_per_line: 5,
        word_chances: vec![
            WordChance { chance: 50, lenght: 4 },
            WordChance { chance: 50, lenght: 5 },
        ]
    }}
}

pub fn load_config() -> Config {
    let cfg = match confy::load_path::<Config>("./config.yaml") {
        Ok(cfg) => cfg,
        Err(_) => panic!("Unable to load the config"),
    };


    if cfg.word_chances.clone().into_iter().fold(0, |a, b| a + b.chance) != 100 {
        panic!("Word chances do not sum up to 100");
    }

    cfg
}
