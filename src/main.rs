use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
mod pre_compute;
mod setup;

fn main() {
    let mut yellow_chars: HashSet<char> = HashSet::new();
    let mut grey_chars: HashSet<char> = HashSet::new();

    grey_chars.insert('b');
    grey_chars.insert('r');
    grey_chars.insert('t');
    grey_chars.insert('s');
    grey_chars.insert('d');
    grey_chars.insert('e');
    grey_chars.insert('m');
    grey_chars.insert('o');
    grey_chars.insert('k');
    grey_chars.insert('e');

    yellow_chars.insert('a');
    yellow_chars.insert('n');
    yellow_chars.insert('l');

    let pre_compute_start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let words: Vec<String> = setup::get_all_five_letter_words();
    let words_data_struct: pre_compute::WorDS = pre_compute::compute(words);

    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("ye to cheating hai")
        .as_millis();
    print!(" precomute time {}\n", start - pre_compute_start);
    let possibilities: HashSet<String> =
        get_word_possibilities(&yellow_chars, &grey_chars, &words_data_struct);
    print!("possibility#:: {}\n", possibilities.len());
    for possibility in possibilities {
        print!("possibility:: {}\n", possibility);
    }

    let guess_start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("ye to cheating hai")
        .as_millis();
    print!(" comute time {}\n", guess_start_time - start);
    let good_guesses: HashSet<String> =
        get_good_guess(&yellow_chars, &grey_chars, &words_data_struct);
    print!("guess:: {}\n", good_guesses.len());
    for guess in good_guesses {
        print!("guess:: {}\n", guess);
    }
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("ye to cheating hai")
        .as_millis();
    print!(" guess compute time {}\n", end - guess_start_time);
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
    for ch in y_chars {
        let char_words: HashSet<&str> =
            hash_to_ref(words_data_struct.consonant_map.get(&ch).clone().unwrap());
        result = match result {
            None => Some(char_words),
            Some(r) => Some(r.intersection(&char_words).cloned().collect()),
        };
    }
    for ch in g_chars {
        let char_words: HashSet<&str> =
            hash_to_ref(words_data_struct.consonant_map.get(&ch).unwrap());
        result = match result {
            None => None,
            Some(r) => Some(r.difference(&char_words).cloned().collect()),
        };
    }
    return hash_to_string(result.unwrap());
}

fn get_good_guess(
    yellow_chars: &HashSet<char>,
    grey_chars: &HashSet<char>,
    word_ds: &pre_compute::WorDS,
) -> HashSet<String> {
    let chars: HashSet<char> = yellow_chars.union(grey_chars).cloned().collect();
    let mut result: Option<HashSet<&str>> =
        Some(hash_to_ref(word_ds.consonant_map.get(&'0').unwrap()));
    match result {
        None => print!("None"),
        Some(ref r) => print!("{}\n", r.len()),
    }
    for ch in chars {
        match result {
            None => print!("None"),
            Some(ref r) => print!("{} removal {}\n", &ch, r.len()),
        }
        let char_words: HashSet<&str> = hash_to_ref(word_ds.consonant_map.get(&ch).unwrap());
        result = match result {
            None => None,
            Some(r) => Some(r.difference(&char_words).cloned().collect()),
        };
    }
    match result {
        None => print!("None"),
        Some(ref r) => print!("now: {}\n", r.len()),
    }
    match result {
        None => HashSet::new(),
        Some(r) => hash_to_string(r),
    }
}
