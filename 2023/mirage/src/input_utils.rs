use std::{collections::VecDeque, env, fs::File, io::{BufRead, BufReader}, path::Path};

use super::models::History;

fn load_lines() -> VecDeque<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("incorrect usage: cargo run -- filepath");
    }
    let path = Path::new(&args[1]);
    let file = match File::open(path) {
        Err(why) => panic!("unable to open file: {}, {}", path.display(), why),
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    let lines: VecDeque<String> = reader
    .lines()
    .map(|l| l.unwrap())
    .collect();

    lines
}

pub fn load_histories() -> Vec<History> {
    let lines = load_lines();
    let mut histories: Vec<History> = Vec::new();
    for line in lines {
        let mut seq: Vec<i32> = Vec::new();
        let split: Vec<&str> = line.split(" ").collect();

        for str_num in split {
            let num = String::from(str_num).parse::<i32>().unwrap();
            seq.push(num);
        }

        histories.push(History::new(seq));
    }
    histories
}