use std::io;

pub enum Input {
    Hirgana = 0,
    Katakana = 1,
}

pub fn ask_for_input() -> Input {
    println!("Syllabary to generate with:");
    println!("   [1] Hiragana");
    println!("   [2] Katakana");
    
    let mut input = String::new();
    while input != "1" && input != "2" {
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                if input.trim() == "1" || input.trim() == "2" {
                    break;
                }
            },
            Err(_) => {},
        }
        
        println!("Incorrect input")
    }
    
    if input.trim() == "1" {
        Input::Hirgana
    } else {
        Input::Katakana
    }
}
