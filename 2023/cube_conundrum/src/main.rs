use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid usage: cargo run -- src/file.txt");
    }

    let path = Path::new(&args[1]);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        // extract information from line
        sum += extract(&line?);
    }

    println!("{}", sum);

    Ok(())
}

static REDMAX: u32 = 12;
static GREENMAX: u32 = 13;
static BLUEMAX: u32 = 14;

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn update(&mut self, shown: (u32, &str)) {
        let (num, color) = shown;
        match color {
            "red" => self.red = if num > self.red { num } else { self.red },
            "green" => self.green = if num > self.green { num } else { self.green },
            "blue" => self.blue = if num > self.blue { num } else { self.blue },
            _ => panic!("invalid color: {}", color),
        }
    }

    fn check(&self) -> bool {
        // println!("red: {}, green: {}, blue: {}", self.red, self.green, self.blue);

        if self.red > REDMAX {
            // println!("red > REDMAX");
            return false
        } else if self.green > GREENMAX {
            // println!("green > GREENMAX");
            return false
        } else if self.blue > BLUEMAX {
            // println!("blue > BLUEMAX");
            return false
        }

        true
    }
}

fn extract(line: &str) -> u32 {
    // find game number
    let line = line.strip_prefix("Game ").unwrap();
    let split_line: Vec<&str> = line.split(':').collect();
    let game_num: u32 = split_line[0].parse::<u32>().unwrap();

    // initialize game struct
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };

    // clean game info
    let game: Vec<&str> = split_line[1].split(';').collect();

    // iterate over every time the elf 'shows' me summa dat
    for set in game {
        let subset: Vec<&str> = set.split(',').collect();
        // store color & num
        for s in subset {
            // remove <space> in subset, then split num & color
            let shown: Vec<&str> = s.strip_prefix(' ').unwrap().split(' ').collect();
            let shown = (shown[0].parse::<u32>().unwrap(), shown[1]);
            // update bag
            bag.update(shown);
        }
    }

    match bag.check() {
        true => return game_num,
        false => return 0,
    }

}