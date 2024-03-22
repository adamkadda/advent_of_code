use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("incorrect usage: cargo run -- file.txt");
    }

    let path = Path::new(&args[1]);

    let file = match File::open(path) {
        Err(_) => panic!("unable to open file: {}", path.display()),
        Ok(f) => f,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
    .lines()
    .map(|l| l.unwrap())
    .collect();

    let mut cards: Vec<Card> = Vec::new();

    for l in lines {
        let mut info: Vec<&str> = l.split(": ").collect();
        info = info[1].split('|').collect();
        let winning: Vec<String> = info[0].trim().split_whitespace().map(|s| s.to_string()).collect();
        let have: Vec<String> = info[1].trim().split_whitespace().map(|s| s.to_string()).collect();

        cards.push(Card::new(winning, have));
    }

    let mut sum: u32 = 0;

    for card in cards {
        sum += card.score();
    }

    println!("{}", sum);
}


struct Card {
    winning: Vec<String>,
    have: Vec<String>,
}

impl Card {
    fn new(winning: Vec<String>, have: Vec<String>) -> Self {
        Self { winning, have }
    }

    fn score(&self) -> u32 {
        let mut tally: u8 = 0;

        let mut win_set: HashSet<u8> = HashSet::new();
        for n in &self.winning {
            // parse to u8
            let num: u8 = match n.parse::<u8>() {
                Ok(val) => val,
                Err(why) => panic!("error parsing {}, {}", n, why),
            };
            win_set.insert(num);
        }

        for n in &self.have {
            let num: u8 = match n.parse::<u8>() {
                Ok(val) => val,
                Err(why) => panic!("error parsing {}, {}", n, why),
            };
            if win_set.contains(&num) {
                tally += 1;
            }
        }

        let mut score: u32 = 0;
        for _ in 0..tally {
            match score {
                0 => score = 1,
                _ => score *= 2,
            }
        }
        score
    }
}