use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::include_str;

/// Simple program to cheat at Wordle
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, required = false, default_value = ".....")]
    green: String,
    #[clap(short, long, required = true, default_value = "")]
    grey: String,
    #[clap(short, long, required = true, default_value = "")]
    yellow: String,
}

fn find_words<'a>(
    word_list: Vec<&'a str>,
    regex_str: &str,
    yellow: &str,
    green: &str,
) -> Option<Vec<&'a str>> {
    let exclusions = Regex::new(regex_str).unwrap();
    let inclusions = Regex::new(green).unwrap();
    let mut results: Vec<&str> = vec![];
    for word in word_list {
        if exclusions.is_match(&word.to_lowercase())
            && inclusions.is_match(&word.to_lowercase())
            && yellow
                .chars()
                .filter(|c| c.is_alphanumeric() && !c.is_ascii_digit())
                .all(|c| word.contains(c))
        {
            results.push(word);
        }
    }
    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

fn construct_exclusion_regex(grey: &str, yellow: &str) -> String {
    let re = yellow.to_owned();
    lazy_static! {
        static ref YELLOW_LETTER_REGEX: Regex = Regex::new(r"\[([a-z]*)\]").unwrap();
    }
    let s = YELLOW_LETTER_REGEX.replace_all(&re, |caps: &regex::Captures| {
        format!("[^{}{}]", &caps[1], grey)
    });
    s.to_string()
}

fn main() {
    let args = Args::parse();
    let five_letter_words = include_str!("five-letter-words");
    let word_list = five_letter_words.split('\n').collect();
    let regex_str: String = construct_exclusion_regex(&args.grey, &args.yellow);
    let results = find_words(word_list, &regex_str, &args.yellow, &args.green);
    match results {
        Some(words) => {
            for word in words {
                println!("{}", word.to_lowercase());
            }
        }
        None => println!("No results"),
    }
}
