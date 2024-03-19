use std::env;
use std::io::{BufRead, BufReader};
use std::path::Path;
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
    
    let mut sum: u32 = 0;
    
    let mut schema: Vec<String> = reader
        .lines()
        .map(|l|l.unwrap())
        .collect();

    // add dummy '.' filled elements at start and end
    let len = schema[0].len();
    let mut dummy = String::new();
    for _ in 0..len {
        dummy.push('.');
    }

    // insert dummy clone at index 0
    schema.insert(0, dummy.clone());
    // reassign len after inserting dummy at index 0
    let len = schema.len();
    // insert dummy at index len
    schema.insert(len, dummy);

    for i in 0..len {
        if i == 0 {
            continue;
        }
        sum += parse(&schema[i - 1], &schema[i], &schema[i + 1]);
    }

    println!("{}", sum);
}


struct Gear {
    top_copy: String,
    mid_copy: String,
    bot_copy: String,

    first: Option<usize>,
    last: Option<usize>,
}

impl Gear {
    // inits first
    fn create(&mut self, index: usize) {

        self.first = Some(index);
        self.last = Some(index);
    }

    fn update(&mut self, index: usize) {
        match self.last {
            Some(_) => self.last = Some(index),
            None => {
                panic!("invalid update attempt at index: {}\n
                at line:\n{}", index, self.mid_copy);
            }
        }
        // dbg!(self.first.unwrap());
    }

    fn check(&self) -> bool {
        // unpack indices
        let start = match self.first {
            Some(i) => i,
            None => panic!("unable to unpack 'first' variable"),
        };
        // dbg!(self.first.unwrap());

        let end = match self.last {
            Some(i) => {
                if i == self.mid_copy.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => panic!("unable to unpack 'last' variable"),
        };
        // dbg!(self.last.unwrap());

        let start = match start {
            0 => start,
            _ => start - 1,
        };

        /* println!("{}", &self.top_copy[start..=end]);
        println!("{}", &self.mid_copy[start..=end]);
        println!("{}", &self.bot_copy[start..=end]);
        println!(""); */

        // check top
        let mut slice = &self.top_copy[start..=end];
        for c in slice.chars() {
            if c == '.' {
                continue;
            } else if c.is_digit(10) {
                continue;
            } else {
                return true;
            }
        }

        // check mid
        slice = &self.mid_copy[start..=end];
        for c in slice.chars() {
            // dbg!(c);
            if c == '.' {
                continue;
            } else if c.is_digit(10) {
                continue;
            } else {
                return true;
            }
        }
        
        // check bot
        slice = &self.bot_copy[start..=end];
        for c in slice.chars() {
            if c == '.' {
                continue;
            } else if c.is_digit(10) {
                continue;
            } else {
                return true;
            }
        }

        false
    }

    fn convert(&self) -> u32 {
        // unpack indices
        let start = match self.first {
            Some(i) => i,
            None => panic!("unable to unpack 'first' variable"),
        };

        let end = match self.last {
            Some(i) => i,
            None => panic!("unable to unpack 'last' variable"),
        };

        let mut num = String::new();
        let slice = &self.mid_copy[start..=end];
        for c in slice.chars() {
            num.push(c);
        }

        match num.parse::<u32>() {
            Ok(n) => return n,
            Err(why) => panic!("error parsing {}, {}", num, why),
        }
    }

    fn reset(&mut self) {
        self.first = None;
        self.last = None;
    }
}

fn parse(top: &str, mid: &str, bot: &str) -> u32 {
    let mut state: bool = false;
    let mut sum: u32 = 0;

    let mut gear = Gear {
        top_copy: top.to_string(),
        mid_copy: mid.to_string(),
        bot_copy: bot.to_string(),

        first: None,
        last: None,
    };

    // iterate for every char
    for (i, c) in mid.char_indices() {
        if c.is_digit(10) {
            // println!("{} is digit", c);
            match state {
                true => {
                    gear.update(i); // update last
                }
                false => {
                    state = true; // open state
                    gear.create(i); // create number
                }
            }
        } else {
            // println!("{} is not digit", c);
            match state {
                true => {
                    if gear.check() == true { // check if valid
                        let n = gear.convert();
                        sum += n; // parse slice to int
                    }
                    gear.reset(); // reset first & last
                    state = false; // close state
                }
                false => (),
            }
        }
    }

    if state == true {
        if gear.check() == true { // check if valid
            let n = gear.convert();
            sum += n; // parse slice to int
        }
    }

    sum
}