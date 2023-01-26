pub mod kana;
pub mod config;

fn main() {
    dbg!(kana::load_katakana());
    let cfg = config::load_config();
    dbg!(cfg);
}
