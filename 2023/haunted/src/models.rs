use std::collections::HashMap;

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
}