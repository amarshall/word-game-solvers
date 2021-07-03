use onig::Regex;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Chain<'a> = Vec<&'a str>;
type Side = Vec<String>;

struct Config {
    depth: usize,
    sides: Vec<Side>,
}

struct State {
    words: Vec<String>,
    char_index: HashMap<char, Vec<usize>>,
}

impl State {
    fn new(words: Vec<String>) -> Self {
        let mut char_index: HashMap<char, Vec<usize>> = HashMap::new();
        for (idx, word) in words.iter().enumerate() {
            let chr = word.chars().next().unwrap();
            if let Some(char_words) = char_index.get_mut(&chr) {
                char_words.push(idx);
            } else {
                char_index.insert(chr, vec![idx]);
            }
        }
        State { words, char_index }
    }

    fn words_starting_with(&self, chr: &char) -> Vec<&str> {
        self.char_index
            .get(chr)
            .unwrap_or(&vec![])
            .iter()
            .map(|idx| self.words.get(*idx).unwrap().as_ref())
            .collect()
    }
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

fn read_valid_words(sides: &[Side]) -> State {
    let file = File::open("/usr/share/dict/words").unwrap();
    let reader = BufReader::new(file);
    let re = build_regex(sides);

    let mut in_count = 0;

    let words = reader
        .lines()
        .inspect(|_| in_count += 1)
        .map(|line| line.unwrap())
        .filter(|line| is_valid_word(&re, &line))
        .collect();
    let state = State::new(words);

    let out_count = state.words.len();
    log::info!(
        "Filtered dictionary of {} words to {} words",
        in_count,
        out_count,
    );
    state
}

fn permute<'a>(state: &'a State, chain: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let last_chr = chain.last().unwrap().chars().last().unwrap();
    state
        .words_starting_with(&last_chr)
        .iter()
        .map(|next_word| {
            let mut c = chain.clone();
            c.push(next_word);
            c
        })
        .collect()
}

fn solve<'a>(state: &'a State, config: &Config) -> Vec<Chain<'a>> {
    let chrs = sides_to_chars(&config.sides);
    let mut chains: Vec<Chain> = state.words.iter().map(|x| vec![x.as_ref()]).collect();
    let mut solutions = vec![];
    for depth in 1..=config.depth {
        if depth > 1 {
            chains = chains
                .iter()
                .flat_map(|chain| permute(&state, chain))
                .collect();
        }
        log::info!("Expanding {} chains at depth {}...", chains.len(), depth);
        solutions.par_extend(chains
            .par_iter()
            .filter(|chain| is_complete_chain(&chrs, chain))
            .map(|v| v.to_owned())
        );
        log::info!("Found {} solutions at depth {}", solutions.len(), depth);
    }
    solutions
}

fn sides_to_chars(sides: &[Side]) -> HashSet<char> {
    sides
        .iter()
        .flatten()
        .map(|s| s.chars())
        .flatten()
        .collect()
}

fn is_complete_chain(chrs: &HashSet<char>, chain: &[&str]) -> bool {
    let mut chrs = chrs.clone();
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
    let state = read_valid_words(&config.sides);
    solve(&state, &config)
        .iter()
        .for_each(|chn| println!("{}", chn.join(" ")));
}
