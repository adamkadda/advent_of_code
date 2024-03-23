use std::collections::HashSet;
use std::{env, usize};
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

        cards.push(Card::new(winning, have, 1));
    }

    let mut sum: u32 = 0;

    for i in 0..cards.len() {
        let tally = cards[i].count();
        let q: u32 = cards[i].quantity;

        let start: usize = i + 1;
        let end: usize = start + tally as usize;

        for j in start..end {
            cards[j].increment(q);
        }

        sum += q;
    }

    println!("{}", sum);
}

#[derive(Debug)]
struct Card {
    winning: Vec<String>,
    have: Vec<String>,
    quantity: u32,
}

impl Card {
    fn new(winning: Vec<String>, have: Vec<String>, quantity: u32) -> Self {
        Self { winning, have, quantity }
    }

    fn count(&self) -> u8 {
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
        tally
    }

    fn increment(&mut self, q: u32) {
        self.quantity += q;
    }
}