use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash)]
pub enum VowelPair {
    AA,
    AE,
    AI,
    AO,
    AU,
    EE,
    EI,
    EO,
    EU,
    II,
    IO,
    IU,
    OO,
    OU,
    UU,
}

enum Vowel {
    A,
    E,
    I,
    O,
    U,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Consonant {
    ALL,
    B,
    C,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
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
            _ => Err(()),
        }
    }
}

pub struct WorDS {
    pub consonant_map: HashMap<Consonant, HashSet<String>>,
    pub vowel_map: HashMap<VowelPair, HashSet<String>>,
}

pub fn compute(words: Vec<String>) -> WorDS {
    let mut word_ds = WorDS {
        vowel_map: HashMap::new(),
        consonant_map: HashMap::new(),
    };
    for word in &words {
        if word.len() != 5 {
            panic!("ye to cheating hai!");
        }
        let mut a = false;
        let mut e = false;
        let mut i = false;
        let mut o = false;
        let mut u = false;
        for ch in word.chars() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
                let double_vowels: HashSet<VowelPair> = get_double_vowels(ch, a, e, i, o, u);
                for vp in double_vowels {
                    match word_ds.vowel_map.get_mut(&vp) {
                        None => {
                            let mut word_set: HashSet<String> = HashSet::new();
                            word_set.insert(word.to_string());
                            word_ds.vowel_map.insert(vp, word_set);
                        }
                        Some(word_set) => {
                            word_set.insert(word.to_string());
                        }
                    }
                }
            }
            match ch {
                'a' => {
                    a = true;
                }
                'e' => {
                    e = true;
                }
                'i' => {
                    i = true;
                }
                'o' => {
                    o = true;
                }
                'u' => {
                    u = true;
                }
                consonant_ch => {
                    let consonant: Consonant =
                        Consonant::from_str(&consonant_ch.to_string()).unwrap();
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
        }
    }
    word_ds
        .consonant_map
        .insert(Consonant::ALL, words.into_iter().collect());
    return word_ds;
}

fn get_double_vowels(
    curr_char: char,
    a: bool,
    e: bool,
    i: bool,
    o: bool,
    u: bool,
) -> HashSet<VowelPair> {
    let mut double_vowels: HashSet<VowelPair> = HashSet::new();
    if a {
        match curr_char {
            'a' => {
                double_vowels.insert(VowelPair::AA);
            }
            'e' => {
                double_vowels.insert(VowelPair::AE);
            }
            'i' => {
                double_vowels.insert(VowelPair::AI);
            }
            'o' => {
                double_vowels.insert(VowelPair::AO);
            }
            'u' => {
                double_vowels.insert(VowelPair::AU);
            }
            _other => panic!("ye to cheating hai"),
        }
    }
    if e {
        match curr_char {
            'a' => {
                double_vowels.insert(VowelPair::AE);
            }
            'e' => {
                double_vowels.insert(VowelPair::EE);
            }
            'i' => {
                double_vowels.insert(VowelPair::EI);
            }
            'o' => {
                double_vowels.insert(VowelPair::EO);
            }
            'u' => {
                double_vowels.insert(VowelPair::EU);
            }
            _other => panic!("ye to cheating hai"),
        }
    }
    if i {
        match curr_char {
            'a' => {
                double_vowels.insert(VowelPair::AI);
            }
            'e' => {
                double_vowels.insert(VowelPair::EI);
            }
            'i' => {
                double_vowels.insert(VowelPair::II);
            }
            'o' => {
                double_vowels.insert(VowelPair::IO);
            }
            'u' => {
                double_vowels.insert(VowelPair::IU);
            }
            _other => panic!("ye to cheating hai"),
        }
    }
    if o {
        match curr_char {
            'a' => {
                double_vowels.insert(VowelPair::AO);
            }
            'e' => {
                double_vowels.insert(VowelPair::EO);
            }
            'i' => {
                double_vowels.insert(VowelPair::IO);
            }
            'o' => {
                double_vowels.insert(VowelPair::OO);
            }
            'u' => {
                double_vowels.insert(VowelPair::OU);
            }
            _other => panic!("ye to cheating hai"),
        }
    }
    if u {
        match curr_char {
            'a' => {
                double_vowels.insert(VowelPair::AU);
            }
            'e' => {
                double_vowels.insert(VowelPair::EU);
            }
            'i' => {
                double_vowels.insert(VowelPair::IU);
            }
            'o' => {
                double_vowels.insert(VowelPair::OU);
            }
            'u' => {
                double_vowels.insert(VowelPair::UU);
            }
            _other => panic!("ye to cheating hai"),
        }
    }
    return double_vowels;
}
