use std::collections::HashMap;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Usage: <file-path>");
        return;
    }

    let file_path = &args[1];
    // println!("{}", file_path);

    let content = fs::read_to_string(file_path).expect("Unable to read from file");

    // Preprocessing to remove punctuations and converting to lowercase
    let re = Regex::new(r"[,.?!']").unwrap();
    let clean_text = re.replace_all(&content, "").to_lowercase();

    let mut words_count: HashMap<String, u32> = HashMap::new();

    let words = clean_text.split_whitespace();
    for word in words {
        let count = words_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for (word, count) in words_count.iter() {
        println!("{}\t--->\t{}",word,count);
    }
}