#[derive(Debug)]
pub struct History {
    sequences: Vec<Vec<i32>>,
}

impl History {
    pub fn new(top: Vec<i32>) -> Self {
        let mut sequences: Vec<Vec<i32>> = Vec::new();
        sequences.push(top);

        while !Self::are_we_there_yet(&sequences) {
            sequences.push(Self::find_diffs(&sequences));
        }

        Self { sequences }
    }

    fn are_we_there_yet(sequences: &Vec<Vec<i32>>) -> bool {
        // supposedly checks bottom-most layer
        let seq = &sequences[sequences.len() - 1];
        for i in seq {
            if *i != 0 { return false; }
        }
        true
    }

    fn find_diffs(sequences: &Vec<Vec<i32>>) -> Vec<i32> {
        // returns the next layer of differences
        let seq = &sequences[sequences.len() - 1];
        let mut new: Vec<i32> = Vec::new();

        let len = seq.len();
        for i in 1..len {
            let value: i32 = seq[i] - seq[i - 1];
            new.push(value);
        }

        new
    }

    pub fn forecast(&self) -> i32 {
        let mut next: i32 = 0;
        for seq in &self.sequences {
            /* next = seq.last().unwrap() + diff;
            diff = next; */
            next += seq.last().unwrap();
        }
        next
    }

    pub fn backcast(&self) -> i32 {
        let mut prev: i32 = 0;
        let mut diff: i32 = 0;
        for seq in self.sequences.iter().rev() {
            /* dbg!(seq);
            println!("{} += {} - {}", prev, seq.first().unwrap(), diff); */
            prev = seq.first().unwrap() - diff;
            diff = prev;
        }
        prev
    }
}