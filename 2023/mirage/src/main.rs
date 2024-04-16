use mirage::input_utils::load_histories;

fn main() {
    let histories = load_histories();
    let mut sum: i32 = 0;
    for hist in histories {
        sum += hist.backcast();
    }
    println!("{}", sum);
}