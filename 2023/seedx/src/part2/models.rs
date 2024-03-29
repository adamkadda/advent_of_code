#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Range {
    pub start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy)]
pub struct Row {
    pub(crate) dest: i64,
    pub(crate) src: i64,
    pub(crate) range: i64,
}

impl Row {
    pub fn new(line: &mut String) -> Self {
        // error handling necessary here?
        let info: Vec<i64> = line
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

        Self {
            dest: info[0],
            src: info[1],
            range: info[2],
        }
    }
}

pub struct Map {
    rows: Vec<Row>,
}

impl Map {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn sort(&mut self) {
        let mut len = self.rows.len();
        let mut swapped = true;
        
        while swapped {
            for i in 0..len {
                swapped = false;

                if i == 0 {
                    continue;
                } else if self.rows[i].src < self.rows[i - 1].src {
                    let temp = self.rows[i];
                    self.rows[i] = self.rows[i - 1];
                    self.rows[i - 1] = temp;
                    swapped = true;
                }

                if !swapped {
                    break;
                }

                len -= 1;
            }
        }
    }

    // call only on a sorted map
    pub fn process(&self, range: &Range) -> Vec<Range> { 
        let mut ranges: Vec<Range> = Vec::new();
        let (mut start, end) = (range.start, range.end);

        // attempt to fit into defined map bounds
        for row in &self.rows {
            let row_end = row.src + row.range - 1;
            let diff = row.dest - row.src;

            let start_state = start >= row.src && start <= row_end;
            let end_state = end >= row.src && end <= row_end;

            match (start_state, end_state) {
                (true, true) => { // start in && end in
                    let xstart = start + diff;
                    let xend = end + diff;
                    ranges.push(Range::new(xstart, xend));
                    return ranges;
                }
                (true, false) => { // start in && end out
                    let xstart = start + diff;
                    let xend = row_end + diff;
                    ranges.push(Range::new(xstart, xend));
                    start = row_end + 1;
                }
                (false, true) => { // start out && end in
                    let xstart = row.src + diff;
                    let xend = end + diff;
                    ranges.push(Range::new(start, row.src - 1));
                    ranges.push(Range::new(xstart, xend));
                    return ranges;
                }
                (false, false) => { // start out && end out
                    if start < row.src && end > row_end {
                        let xstart = row.src + diff;
                        let xend = row_end + diff;
                        ranges.push(Range::new(start, row.src - 1));
                        ranges.push(Range::new(xstart, xend));
                        start = row_end + 1;
                    }
                }
            }
        }
        // range falls outside defined map bounds
        ranges.push(Range::new(start, end));
        ranges
    }
}