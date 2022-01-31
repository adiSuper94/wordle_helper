use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub enum Consonant {
    ALL,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl FromStr for Consonant {
    type Err = ();
    fn from_str(input: &str) -> Result<Consonant, Self::Err> {
        let ch: char = input.to_lowercase().chars().nth(0).unwrap();
        match ch {
            'b' => Ok(Consonant::B),
            'c' => Ok(Consonant::C),
            'd' => Ok(Consonant::D),
            'f' => Ok(Consonant::F),
            'g' => Ok(Consonant::G),
            'h' => Ok(Consonant::H),
            'j' => Ok(Consonant::J),
            'k' => Ok(Consonant::K),
            'l' => Ok(Consonant::L),
            'm' => Ok(Consonant::M),
            'n' => Ok(Consonant::N),
            'p' => Ok(Consonant::P),
            'q' => Ok(Consonant::Q),
            'r' => Ok(Consonant::R),
            's' => Ok(Consonant::S),
            't' => Ok(Consonant::T),
            'v' => Ok(Consonant::V),
            'w' => Ok(Consonant::W),
            'x' => Ok(Consonant::X),
            'y' => Ok(Consonant::Y),
            'z' => Ok(Consonant::Z),
            'a' => Ok(Consonant::A),
            'e' => Ok(Consonant::E),
            'i' => Ok(Consonant::I),
            'o' => Ok(Consonant::O),
            'u' => Ok(Consonant::U),
            _ => Err(()),
        }
    }
}

pub struct WorDS {
    pub consonant_map: HashMap<Consonant, HashSet<String>>,
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
            let consonant: Consonant = Consonant::from_str(&ch.to_string()).unwrap();
            match word_ds.consonant_map.get_mut(&consonant) {
                None => {
                    let mut word_set: HashSet<String> = HashSet::new();
                    word_set.insert(word.to_string());
                    word_ds.consonant_map.insert(consonant, word_set);
                }
                Some(word_set) => {
                    word_set.insert(word.to_string());
                }
            };
        }
    }
    word_ds
        .consonant_map
        .insert(Consonant::ALL, words.into_iter().collect());
    return word_ds;
}
