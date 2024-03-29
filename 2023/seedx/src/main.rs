use input_utils::load_input;
use part2::{get_structs, merge};

mod input_utils;
mod part2;

fn main() {
    let lines = load_input();
    let (mut ranges, mut maps) = get_structs(lines);

    for map in &mut maps {
        map.sort();
        let mut temp: Vec<_> = Vec::new();
        for range in &ranges {
            temp.push(map.process(range));
        }
        ranges = merge(temp);
    }

    let mut min: i64 = ranges[0].start;
    for range in ranges {
        if range.start < min {
            min = range.start;
        }
    }
    println!("{}", min); // not 88637325, 73455403, 
}