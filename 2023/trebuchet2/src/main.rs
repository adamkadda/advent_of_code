use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("incorrect usage: cargo run -- src/file.txt");
    }

    let path = Path::new(&args[1]);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        sum += parse(line?);
    }

    println!("{}", sum);
 
    Ok(())
}


fn parse(string: String) -> u32{
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    
    // iterate over every char
    for (index, character) in string.char_indices() {
        match character {
            // digit case
            '0'..='9' => {
                if first == None {
                    first = Some(character);
                }
                last = Some(character);
            }
            // possible word case
            _ => {
                if let Some(c) = evaluate(&string[index..]) {
                    if first == None {
                        first = Some(c);
                    }
                    last = Some(c);
                }
            }
        }
    }

    let mut calib = String::new();
    calib.push(first.unwrap());
    calib.push(last.unwrap());

    calib.parse::<u32>().unwrap()

}


fn evaluate(slice: &str) -> Option<char> {
    let digits: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];

    // check if slice starts with any digits
    for (word, c) in digits {
        if slice.starts_with(word) {
            return Some(c);
        }
    }

    None
}