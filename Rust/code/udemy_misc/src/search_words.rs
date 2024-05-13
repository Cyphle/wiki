use std::collections::HashMap;

pub fn word_grouping(words_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash = HashMap::new();

    let mut char_freq = vec![0; 26];

    for current_word in words_list {
        for c in current_word.to_lowercase().chars() {
            char_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }

        let key: String = char_freq.into_iter().map(|i| i.to_string()).collect::<String>();
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);

        char_freq = vec![0; 26];
    }

    for (key, value) in &word_hash {
        println!("key # {:?} value {:?}", key, value);
    }

    word_hash.into_iter().map(|(_, v)| v).collect()
}

pub fn search_word() {
    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "studpi".to_string(),
        "apple".to_string(),
        "appel".to_string()
    ];

    let grouping = word_grouping(words);

    let input_word = String::from("teh");
    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The group of the word is {:?}", i);
        }
    }
}