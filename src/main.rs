use onig::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Side = Vec<String>;
type WordIndex = HashMap<char, Vec<String>>;

struct Config {
    depth: usize,
    sides: Vec<Side>,
}

fn build_regex(sides: &[Side]) -> Regex {
    let inner_pattern = sides
        .iter()
        .map(|side| {
            let chars = side.join("");
            format!("[{}](?![{}])", chars, chars)
        })
        .collect::<Vec<String>>()
        .join("|");
    let pattern = format!("^({})+$", inner_pattern);
    Regex::new(&pattern).unwrap()
}

fn is_valid_word(re: &Regex, word: &str) -> bool {
    re.is_match(word)
}

fn read_valid_words(sides: &[Side]) -> Vec<String> {
    let file = File::open("/usr/share/dict/words").unwrap();
    let reader = BufReader::new(file);
    let re = build_regex(sides);

    let mut in_count = 0;

    let words = reader
        .lines()
        .inspect(|_| in_count += 1)
        .map(|line| line.unwrap())
        .filter(|line| is_valid_word(&re, &line))
        .collect::<Vec<String>>();

    let out_count = words.len();
    log::info!(
        "Filtered dictionary of {} words to {} words",
        in_count,
        out_count
    );
    words
}

fn build_index(words: Vec<String>) -> WordIndex {
    let mut word_index: WordIndex = HashMap::new();
    for word in words {
        let chr = word.chars().next().unwrap();
        if let Some(idx) = word_index.get_mut(&chr) {
            idx.push(word);
        } else {
            word_index.insert(chr, vec![word]);
        }
    }
    word_index
}

fn chain<'a>(idx: &'a WordIndex, word: &'a str, depth: usize) -> Vec<Vec<&'a str>> {
    if depth == 1 {
        return vec![vec![word]];
    }
    let last_chr = word.chars().last().unwrap();
    idx.get(&last_chr)
        .unwrap()
        .iter()
        .flat_map(|next_word| chain(idx, next_word, depth - 1))
        .map(|chn| {
            let mut c = chn.clone();
            c.insert(0, word);
            c
        })
        .collect()
}

fn sides_to_chars(sides: &[Side]) -> HashSet<char> {
    sides
        .iter()
        .flatten()
        .map(|s| s.chars())
        .flatten()
        .collect()
}

fn is_complete_chain(sides: &[Side], chain: &[&str]) -> bool {
    let mut chrs = sides_to_chars(sides);
    for chr in chain.iter().map(|s| s.chars()).flatten() {
        chrs.remove(&chr);
    }
    chrs.is_empty()
}

fn argparse() -> Config {
    let matches = clap::App::new("Letter Boxed Solver")
        .arg(
            clap::Arg::with_name("depth")
                .long("depth")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("sides")
                .long("sides")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    Config {
        depth: matches.value_of("depth").unwrap().parse::<usize>().unwrap(),
        sides: parse_sides(matches.value_of("sides").unwrap()),
    }
}

fn parse_sides(s: &str) -> Vec<Side> {
    s.split(',')
        .map(|chrs| chrs.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| writeln!(buf, "[{}] {}", record.level(), record.args()))
        .init();
    let config = argparse();
    let word_index = build_index(read_valid_words(&config.sides));
    word_index
        .values()
        .flatten()
        .flat_map(|word| chain(&word_index, &word, config.depth))
        .filter(|chn| is_complete_chain(&config.sides, &chn))
        .for_each(|chn| println!("{}", chn.join(" ")));
}
