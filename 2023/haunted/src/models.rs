use prime_factorization::Factorization;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Journey {
    instructions: Vec<Direction>,
    network: HashMap<String, (String, String)>,
}

impl Journey {
    pub fn new(instructions: Vec<Direction>, network: HashMap<String, (String, String)>) -> Self {
        Self { instructions, network }
    }

    pub fn travel(&self) -> usize {
        let start= String::from("AAA");
        let dest = String::from("ZZZ");

        let mut loc: &str = &start;
        let mut steps: usize = 0;

        while loc != dest {
            for mv in &self.instructions {
                loc = match mv {
                    Direction::Left => &self.network.get(loc).unwrap().0,
                    Direction::Right => &self.network.get(loc).unwrap().1,
                };
                steps += 1;
            }
        }
        steps
    }

    pub fn escape(&self) {
        let mut paths: Vec<String> = Vec::new();

        for key in self.network.keys() {
            if key.ends_with("A") { paths.push(key.clone()); }
        }

        let mut ttfz: Vec<u32> = Vec::new();

        for path in paths {
            let mut steps: u32 = 0;
            let mut cur: &str = &path; 
            'outer: loop { // 
                for mv in &self.instructions {
                    cur = match mv { // traverse network, move to the next node
                        Direction::Left => &self.network.get(cur).unwrap().0,
                        Direction::Right => &self.network.get(cur).unwrap().1,
                    };
                    steps += 1; // increment steps
                    if cur.ends_with("Z") { // first Z has been reached
                        break 'outer;
                    }
                }
            }
            ttfz.push(steps);
        }

        // I used prime factorization to find the LCM

        for num in ttfz {
            let factor_repr = Factorization::run(num);
            dbg!(factor_repr.factors);
        }
    }
}