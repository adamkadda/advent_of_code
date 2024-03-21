use std::{env, usize};
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
    let mut schema: Vec<String> = reader
        .lines()
        .map(|l|l.unwrap())
        .collect();
    
    for line in &mut schema {
        line.push('.');
        line.insert(0, '.');
    }

    let len = schema[0].len(); // len of line
    let mut dummy = String::new();
    for _ in 0..len {
        dummy.push('.');
    }
    schema.push(dummy.clone());
    schema.insert(0, dummy);

    let mut sum: u32 = 0;

    let len = schema.len() - 1;
    for i in 1..len {
        sum += parse(&schema[i - 1], &schema[i], &schema[i + 1]);
    }

    println!("{}", sum);

}

fn parse(top: &str, mid: &str, bot: &str) -> u32 {
    let mut sum: u32 = 0;
    for (i, c) in mid.char_indices() {
        if c == '*' {
            let mut outputs: Vec<LayerOutput> = Vec::new();
            outputs.push(check_slice(i, top));
            outputs.push(check_slice(i, mid));
            outputs.push(check_slice(i, bot));

            let mut gear1: u32 = 0;
            let mut gear2: u32 = 0;

            let tally = outputs[0].tally + outputs[1].tally + outputs[2].tally;
            if tally != 2 {
                continue;
            }

            for output in outputs {
                for index in 0..2 {
                    let num = output.nums[index].clone();
                    if num.is_empty() {
                        continue
                    }
                    match gear1 {
                        0 => gear1 = num.parse::<u32>().expect("parsing error"),
                        _ => gear2 = num.parse::<u32>().expect("parsing error"),
                    }
                }
            }
            sum += gear1 * gear2;
        }
    }
    sum
}

#[derive(Debug)]
struct LayerOutput {
    tally: usize,
    nums: [String; 2],
    row: String,
}

impl LayerOutput {
    fn init_num(&mut self, i: usize) { // determine whether char belongs to first or second number
        if self.nums[0].is_empty() {
            self.seek_chars(i, 0); // belongs to first => n = 0
        } else {
            self.seek_chars(i, 1); // belongs to second => n = 1
        }
    }


    fn seek_chars(&mut self, i: usize, n: usize) { // find the rest of the number
        let len = self.row.len();
        // check right-side
        let mut slice = &self.row[i..len];
        for c in slice.chars() {
            if c.is_digit(10) {
                self.nums[n].push(c);
            } else {
                break;
            }
        }

        // check left-side
        slice = &self.row[0..i];
        for c in slice.chars().rev() {
            if c.is_digit(10) {
                self.nums[n].insert(0, c);
            } else {
                break;
            }
        }
    }
}


fn check_slice(i: usize, layer: &str) -> LayerOutput {
    let mut output = LayerOutput {
        tally: 0,
        nums: [String::new(), String::new()],
        row: layer.to_string(),
    };
    
    let (start, end) = (i - 1, i + 1);
    let slice: &str = &layer[start..=end];

    let mut state = false;

    for (x, c) in slice.char_indices() {
        if c.is_digit(10) && state == false { // new digit encountered
            state = true;
            output.tally += 1;

            let found = if x == 0 {
                i - 1
            } else if x == 1 {
                i
            } else {
                i + 1
            };

            output.init_num(found);
        } else if !c.is_digit(10) && state == true { // previous digit just ended
            state = false;
        } // else, either mid digit, or no 'nearby' digits
    }

    output
}