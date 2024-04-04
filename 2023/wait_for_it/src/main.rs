mod input_utils;
mod parts;

use input_utils::load_input;
// use parts::solve_part1;
use parts::Race;

fn main() {
    // solve_part1();

    let lines = load_input();

    let mut cur = String::new();

    for c in lines[0].chars() {
        if c.is_digit(10) {cur.push(c)};
    }
    let time = cur.parse::<f64>().unwrap();
    
    cur.clear();

    for c in lines[1].chars() {
        if c.is_digit(10) {
            cur.push(c);
        }
    }
    let dist = cur.parse::<f64>().unwrap();

    let race = Race::new(dist, time);
    let ways = race.ways_to_win();
    println!("{}", ways);
}