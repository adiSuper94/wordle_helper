use std::collections::HashMap;
use std::collections::HashSet;

pub struct WorDS {
    pub consonant_map: HashMap<char, HashSet<String>>,
}

pub fn compute(words: Vec<String>) -> WorDS {
    let mut word_ds = WorDS {
        consonant_map: HashMap::new(),
    };
    for word in &words {
        if word.len() != 5 {
            panic!("ye to cheating hai!");
        }
        for ch in word.chars() {
            match word_ds.consonant_map.get_mut(&ch) {
                None => {
                    let mut word_set: HashSet<String> = HashSet::new();
                    word_set.insert(word.to_string());
                    word_ds.consonant_map.insert(ch, word_set);
                }
                Some(word_set) => {
                    word_set.insert(word.to_string());
                }
            };
        }
    }
    word_ds
        .consonant_map
        .insert('0', words.into_iter().collect());
    return word_ds;
}
