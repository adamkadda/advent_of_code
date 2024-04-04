use crate::input_utils::load_input;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Race {
    dist: f64,
    time: f64,
}

#[allow(dead_code)]
impl Race {
    pub fn new(dist: f64, time: f64) -> Self {
        Race { dist, time }
    }

    pub fn ways_to_win(&self) -> u64 {
        let ways = match self.time % 2.0 == 0.0 {
            true => {
                let optimal = (self.time / 2.0) - 1.0;
                (2.0 * (optimal - self.left())) + 1.0
            }
            false => {
                let optimal = (self.time / 2.0).floor();
                2.0 * (optimal - self.left())
            }
        };

        ways as u64

    }

    pub fn left(&self) -> f64 {
        ((-1.0 * self.time + (self.time.powi(2) - 4.0 * self.dist).sqrt()) / -2.0).floor()
    }
}

#[allow(dead_code)]
pub fn solve_part1() {
    let lines = load_input();

    let mut times: Vec<f64> = Vec::new();
    let mut dists: Vec<f64> = Vec::new();

    let mut cur = String::new();
    let mut state = false;

    for c in lines[0].chars() {
        match (c.is_digit(10), state) {
            (true, true) => { // mid num
                cur.push(c);
            }
            (true, false) => { // start of num
                cur.push(c);
                state = true;
            }
            (false, true) => { // end of num
                times.push(cur.parse::<f64>().unwrap());
                cur.clear();
                state = false;
            }
            (false, false) => (),
        }
    }
    times.push(cur.parse::<f64>().unwrap());
    cur.clear();
    state = false;

    for c in lines[1].chars() {
        match (c.is_digit(10), state) {
            (true, true) => { // mid num
                cur.push(c);
            }
            (true, false) => { // start of num
                cur.push(c);
                state = true;
            }
            (false, true) => { // end of num
                dists.push(cur.parse::<f64>().unwrap());
                cur.clear();
                state = false;
            }
            (false, false) => (),
        }
    }
    dists.push(cur.parse::<f64>().unwrap());

    let n = times.len();
    let mut races: Vec<Race> = Vec::new();

    for i in 0..n {
        races.push(Race::new(dists[i], times[i]));
    }

    let mut margin: u64 = 1;

    for race in races {
        margin *= race.ways_to_win();
    }

    println!("{}", margin);
}