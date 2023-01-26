use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub kana_types: Vec<String>,
    pub words_count: u32,
    pub words_per_line: u32,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { 
        kana_types: vec!["gojūon".to_string(), "dakuten".to_string(), "handakuten".to_string(), "yōon".to_string()], 
        words_count: 50,
        words_per_line: 5,
    }}
}

pub fn load_config() -> Config {
    match confy::load_path::<Config>("./config.yaml") {
        Ok(cfg) => cfg,
        Err(_) => panic!("Unable to load the config"),
    }
}
