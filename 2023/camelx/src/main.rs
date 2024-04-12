/* mod part1;
use part1::solve_part1; */

mod input_utils;
use input_utils::load_input;

mod part2;
use part2::{get_hands, compare};

fn main() {
    // solve_part1();
    
    let lines = load_input();
    let mut hands = get_hands(lines);

    hands.sort_by(|a, b| compare(a, b));

    // dbg!(&hands);

    let mut rank: u32 = 1;
    let mut winnings: u32 = 0;

    for hand in hands {
        /* println!("{}", winnings);
        println!("{} * {} = {}", &hand.bid, rank, &hand.bid * rank);
        println!(""); */
        winnings += hand.bid * rank;
        rank += 1;
    }

    // 252782999, too low
    println!("{}", winnings);
}