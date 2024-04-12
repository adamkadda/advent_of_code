mod input_utils;
use input_utils::load_input;

mod part1;
use part1::{get_hands, compare};

fn main() {
    let lines = load_input();
    let mut hands = get_hands(lines);

    hands.sort_by(|a, b| compare(a, b));

    /* dbg!(&hands[54]);
    dbg!(&hands[55]); */

    let mut rank: u32 = 1;
    let mut winnings: u32 = 0;

    for hand in hands {
        /* println!("{}", winnings);
        println!("{} * {} = {}", &hand.bid, rank, &hand.bid * rank);
        println!(""); */
        winnings += hand.bid * rank;
        rank += 1;
    }

    // too large = 252052790
    println!("{}", winnings);
}