mod models;
mod input_utils;
use input_utils::load_journey;

fn main() {
    let journey = load_journey();
    journey.escape();
}