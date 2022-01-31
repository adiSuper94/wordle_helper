use serde_json::{Result, Value};
use std::fs;

pub fn get_all_five_letter_words() -> Vec<String> {
    let dictionary: Value = get_full_dict();
    let mut five_letter_words: Vec<String> = Vec::new();
    for entry in dictionary.as_object().unwrap() {
        let (key, _val) = entry;
        if key.to_string().len() == 5 {
            five_letter_words.push(key.to_string().to_lowercase())
        }
    }
    return five_letter_words;
}

fn get_full_dict() -> Value {
    let json_str: String = fs::read_to_string(
        "/home/adisuper/Work/WordleHelper/wordle_helper/resource/words_dictionary.json",
    )
    .expect("Something went wrong wile reading the file.");
    let result: Result<Value> = serde_json::from_str(&json_str);
    let dict: Value = match result {
        Ok(dictionary) => dictionary,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
    return dict;
}
