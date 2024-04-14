use std::collections::HashMap;
use std::{collections::VecDeque, env, fs::File, io::{BufRead, BufReader}, path::Path};

use crate::models::{Direction, Journey};

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

pub fn load_journey() -> Journey {
    let mut lines = load_lines();

    let mut instructions: Vec<Direction> = Vec::new();

    for ch in lines.pop_front().unwrap().chars() {
        match ch {
            'L' => instructions.push(Direction::Left),
            'R' => instructions.push(Direction::Right),
            inv => panic!("invalid char in first line: '{}'", inv),
        }
    }

    lines.pop_front();

    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let k: String = line[0..3].into();
        let v: (String, String) = (line[7..10].into(), line[12..15].into());
        network.insert(k, v);
    }

    Journey::new(instructions, network)
}