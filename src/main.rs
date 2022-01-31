use std::collections::HashSet;
use std::str::FromStr;
mod pre_compute;
mod setup;

fn main() {
    let mut yellow_chars: HashSet<char> = HashSet::new();
    let mut grey_chars: HashSet<char> = HashSet::new();
    yellow_chars.insert('s');
    yellow_chars.insert('l');
    yellow_chars.insert('e');
    grey_chars.insert('b');
    grey_chars.insert('r');
    grey_chars.insert('d');
    grey_chars.insert('c');
    grey_chars.insert('h');
    grey_chars.insert('g');
    grey_chars.insert('u');
    grey_chars.insert('z');
    grey_chars.insert('k');
    grey_chars.insert('a');
    grey_chars.insert('o');
    grey_chars.insert('v');
    grey_chars.insert('i');
    grey_chars.insert('n');
    grey_chars.insert('w');
    grey_chars.insert('m');
    grey_chars.insert('p');

    let words: Vec<String> = setup::get_all_five_letter_words();
    let words_data_struct: pre_compute::WorDS = pre_compute::compute(words);

    let possibilities: HashSet<String> =
        get_word_possibilities(&yellow_chars, &grey_chars, &words_data_struct);
    print!("possibility#:: {}\n", possibilities.len());
    for possibility in possibilities {
        print!("possibility:: {}\n", possibility);
    }
    let good_guesses: HashSet<String> =
        get_good_guess(&yellow_chars, &grey_chars, &words_data_struct);
    for guess in good_guesses {
        print!("guess:: {}\n", guess);
    }
}
fn hash_to_ref(h: &HashSet<String>) -> HashSet<&str> {
    h.iter().map(|s| s.as_str()).collect()
}
fn hash_to_string(h: HashSet<&str>) -> HashSet<String> {
    h.iter().map(|s| s.to_string()).collect()
}
fn get_word_possibilities(
    y_chars: &HashSet<char>,
    g_chars: &HashSet<char>,
    words_data_struct: &pre_compute::WorDS,
) -> HashSet<String> {
    let mut result: Option<HashSet<&str>> = None;
    let mut a = false;
    let mut e = false;
    let mut i = false;
    let mut o = false;
    let mut u = false;
    for ch in y_chars {
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
                let char_consonant =
                    pre_compute::Consonant::from_str(&consonant_ch.to_string()).unwrap();
                let char_words: HashSet<&str> = hash_to_ref(
                    words_data_struct
                        .consonant_map
                        .get(&char_consonant)
                        .clone()
                        .unwrap(),
                );
                result = match result {
                    None => Some(char_words),
                    Some(r) => Some(r.intersection(&char_words).cloned().collect()),
                };
            }
        }
    }
    let y_vowel_pair: HashSet<pre_compute::VowelPair> = get_vowel_pair_set(a, e, i, o, u);
    for vp in y_vowel_pair {
        let vowel_pair_words: HashSet<&str> =
            hash_to_ref(words_data_struct.vowel_map.get(&vp).unwrap());
        result = match result {
            None => Some(vowel_pair_words),
            Some(r) => Some(vowel_pair_words.difference(&r).cloned().collect()),
        }
    }
    a = false;
    e = false;
    i = false;
    o = false;
    u = false;
    for ch in g_chars {
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
                let char_consonant =
                    pre_compute::Consonant::from_str(&consonant_ch.to_string()).unwrap();
                let char_words: HashSet<&str> = hash_to_ref(
                    words_data_struct
                        .consonant_map
                        .get(&char_consonant)
                        .unwrap(),
                );
                result = match result {
                    None => None,
                    Some(r) => Some(r.difference(&char_words).cloned().collect()),
                };
            }
        }
    }
    let g_vowel_pair: HashSet<pre_compute::VowelPair> = get_vowel_pair_set(a, e, i, o, u);
    for vp in g_vowel_pair {
        let vowel_pair_words: HashSet<&str> =
            hash_to_ref(words_data_struct.vowel_map.get(&vp).unwrap());
        result = match result {
            None => None,
            Some(r) => Some(r.difference(&vowel_pair_words).cloned().collect()),
        }
    }
    return hash_to_string(result.unwrap());
}

fn get_vowel_pair_set(
    a: bool,
    e: bool,
    i: bool,
    o: bool,
    u: bool,
) -> HashSet<pre_compute::VowelPair> {
    let mut y_vowel_pair: HashSet<pre_compute::VowelPair> = HashSet::new();
    if a {
        if e {
            y_vowel_pair.insert(pre_compute::VowelPair::AE);
        }
        if i {
            y_vowel_pair.insert(pre_compute::VowelPair::AI);
        }
        if o {
            y_vowel_pair.insert(pre_compute::VowelPair::AO);
        }
        if u {
            y_vowel_pair.insert(pre_compute::VowelPair::AU);
        }
    }
    if e {
        if i {
            y_vowel_pair.insert(pre_compute::VowelPair::EI);
        }
        if o {
            y_vowel_pair.insert(pre_compute::VowelPair::EO);
        }
        if u {
            y_vowel_pair.insert(pre_compute::VowelPair::EU);
        }
    }
    if i {
        if o {
            y_vowel_pair.insert(pre_compute::VowelPair::IO);
        }
        if u {
            y_vowel_pair.insert(pre_compute::VowelPair::IU);
        }
    }
    if o {
        if u {
            y_vowel_pair.insert(pre_compute::VowelPair::OU);
        }
    }

    return y_vowel_pair;
}

fn get_good_guess(
    yellow_chars: &HashSet<char>,
    grey_chars: &HashSet<char>,
    word_ds: &pre_compute::WorDS,
) -> HashSet<String> {
    let chars: HashSet<char> = yellow_chars.union(&grey_chars).cloned().collect();
    let mut result: Option<HashSet<&str>> = Some(hash_to_ref(
        word_ds
            .consonant_map
            .get(&pre_compute::Consonant::ALL)
            .unwrap(),
    ));
    let mut a = false;
    let mut e = false;
    let mut i = false;
    let mut o = false;
    let mut u = false;
    for ch in chars {
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
                let char_consonant =
                    pre_compute::Consonant::from_str(&consonant_ch.to_string()).unwrap();
                let char_words: HashSet<&str> =
                    hash_to_ref(word_ds.consonant_map.get(&char_consonant).unwrap());
                result = match result {
                    None => None,
                    Some(r) => Some(r.difference(&char_words).cloned().collect()),
                };
            }
        }
    }

    let g_vowel_pair: HashSet<pre_compute::VowelPair> = get_vowel_pair_set(a, e, i, o, u);
    for vp in g_vowel_pair {
        let vowel_pair_words: HashSet<&str> = hash_to_ref(word_ds.vowel_map.get(&vp).unwrap());
        result = match result {
            None => None,
            Some(r) => Some(r.difference(&vowel_pair_words).cloned().collect()),
        }
    }

    match result {
        None => HashSet::new(),
        Some(r) => hash_to_string(r),
    }
}
