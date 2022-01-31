use clap::Parser;
use regex::Regex;
use std::{io::Read, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Regex for the word
    regex: String,

    /// Dictionary
    #[clap(
        short,
        long,
        required = false,
        parse(from_os_str),
        default_value = "/usr/share/dict/words"
    )]
    dictionary: PathBuf,
}

fn find_word<'a>(word_regex: &'a str, word_list: Vec<&'a str>) -> Option<Vec<&'a str>> {
    let re = Regex::new(word_regex).unwrap();
    let mut results: Vec<&str> = vec![];
    for word in word_list {
        if word.len() != 5 {
            continue;
        }
        if re.is_match(&word.to_lowercase()) {
            results.push(word);
        }
    }
    if !results.is_empty() {
        return Some(results);
    }
    None
}

fn main() {
    let args = Args::parse();
    let word_regex = args.regex;
    let mut contents = String::new();
    let mut f = std::fs::File::open(args.dictionary).expect("Unable to read file");
    f.read_to_string(&mut contents).expect("Unable to string");
    let word_list: Vec<&str> = contents.split('\n').collect();
    let result = find_word(&word_regex, word_list);
    match result {
        Some(words) => {
            for word in words {
                println!("{}", word.to_lowercase());
            }
        }
        None => println!("No result"),
    }
}
