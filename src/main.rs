use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
mod pre_compute;
mod setup;

fn main() {
    let mut yellow_chars: HashSet<char> = HashSet::new();
    let mut grey_chars: HashSet<char> = HashSet::new();

    println!("Enter grey chars without space");
    let grey_str: String = std::io::stdin().lines().next().unwrap().unwrap();
    for grey_char in grey_str.chars() {
        grey_chars.insert(grey_char);
    }

    println!("Enter yellow chars without space");
    let yellow_str: String = std::io::stdin().lines().next().unwrap().unwrap();
    if yellow_str.len() > 5 {
        println!("You entered more than 5 yellow chars considering only the first 5");
    }
    let mut i: u16 = 0;
    for yellow_char in yellow_str.chars() {
        i += 1;
        if i > 5 {
            break;
        }
        yellow_chars.insert(yellow_char);
    }
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
    println!(" precomute time {}", start - pre_compute_start);
    let possibilities: HashSet<String> =
        get_word_possibilities(&yellow_chars, &grey_chars, &words_data_struct);
    println!("possibility#:: {}", possibilities.len());
    for possibility in possibilities {
        println!("possibility:: {}", possibility);
    }

    let guess_start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("ye to cheating hai")
        .as_millis();
    println!(" comute time {}", guess_start_time - start);
    let good_guesses: HashSet<String> =
        get_good_guess(&yellow_chars, &grey_chars, &words_data_struct);
    println!("guess:: {}", good_guesses.len());
    for guess in good_guesses {
        println!("guess:: {}", guess);
    }
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("ye to cheating hai")
        .as_millis();
    println!(" guess compute time {}", end - guess_start_time);
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
            hash_to_ref(words_data_struct.consonant_map.get(ch).unwrap());
        result = match result {
            None => Some(char_words),
            Some(r) => Some(r.intersection(&char_words).cloned().collect()),
        };
    }
    for ch in g_chars {
        let char_words: HashSet<&str> =
            hash_to_ref(words_data_struct.consonant_map.get(ch).unwrap());
        result = result.map(|r| r.difference(&char_words).cloned().collect());
    }
    hash_to_string(result.unwrap())
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
        None => println!("None"),
        Some(ref r) => println!("{}\n", r.len()),
    }
    for ch in chars {
        match result {
            None => println!("None"),
            Some(ref r) => println!("{} removal {}\n", &ch, r.len()),
        }
        let char_words: HashSet<&str> = hash_to_ref(word_ds.consonant_map.get(&ch).unwrap());
        result = result.map(|r| r.difference(&char_words).cloned().collect());
    }
    match result {
        None => println!("None"),
        Some(ref r) => println!("now: {}\n", r.len()),
    }
    match result {
        None => HashSet::new(),
        Some(r) => hash_to_string(r),
    }
}
