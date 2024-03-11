use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // turn the output of env::args(), an iterator, into a vector 
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Incorrect usage: cargo run -- src/file.txt");
    }

    // `args[1]` access the second comand-line argument
    // https://doc.rust-lang.org/std/path/struct.Path.html
    // https://doc.rust-lang.org/std/path/struct.Display.html
    let path = Path::new(&args[1]);
    let _display = path.display();

    // https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    // https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html
    let file = File::open(&path)?;

    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    let reader = BufReader::new(file);
    
    let mut sum: u32 = 0;

    // https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    // The iterator returned from this function will yield instances of io::Result<String>
    for line in reader.lines() {
        sum += get_calibration(line?);
    }

    println!("{}", sum);

    Ok(())
}

/* fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
} */

fn get_calibration(line: String) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for c in line.chars() {
        if c.is_digit(10) {
            if first == None {
                first = Some(c);
            }

            last = Some(c);
        }
    }

    let mut calibration = String::new();
    calibration.push(first.unwrap());
    calibration.push(last.unwrap());

    // turbofish
    // https://doc.rust-lang.org/std/primitive.str.html#method.parse
    calibration.parse::<u32>().unwrap()
}