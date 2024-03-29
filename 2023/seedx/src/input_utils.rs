#[allow(dead_code)]
use std::{collections::VecDeque, env, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn load_input() -> VecDeque<String> {
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