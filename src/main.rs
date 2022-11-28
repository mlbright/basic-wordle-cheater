use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;
use std::include_str;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

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

fn greens(word: &str, green: &str) -> bool {
    for i in 0..5 {
        if green.chars().nth(i) == Some('.') {
            continue;
        }
        if word.chars().nth(i) != green.chars().nth(i) {
            return false;
        }
    }
    true
}

fn find_words<'a>(
    word_list: Vec<&'a str>,
    regex_str: &str,
    grey: &str,
    yellow: &str,
    green: &str,
) -> Option<Vec<&'a str>> {
    let exclusions_filter = Regex::new(regex_str).unwrap();
    let mut results: Vec<&str> = vec![];
    for word in word_list {
        if greens(word, green)
            && yellow
                .chars()
                .filter(|c| c.is_alphanumeric() && !c.is_ascii_digit())
                .all(|c| word.contains(c))
            && exclusions_filter.is_match(word)
            && grey
                .chars()
                .filter(|c| c.is_alphanumeric() && !c.is_ascii_digit())
                .all(|c| !word.contains(c))
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
    let re_str = yellow.to_owned();
    let re = regex!(r"\[([a-z]*)\]");
    re.replace_all(&re_str, |caps: &regex::Captures| {
        format!("[^{}{}]", &caps[1], grey)
    })
    .to_string()
}

static FIVE_LETTER_WORDS: Lazy<Vec<&str>> =
    Lazy::new(|| include_str!("five-letter-words").split('\n').collect());

fn main() {
    let args = Args::parse();
    let regex_str: String = construct_exclusion_regex(&args.grey, &args.yellow);
    match find_words(
        FIVE_LETTER_WORDS.to_vec(),
        &regex_str,
        &args.grey,
        &args.yellow,
        &args.green,
    ) {
        Some(words) => {
            for word in words {
                println!("{}", word);
            }
        }
        None => println!("No results"),
    }
}
