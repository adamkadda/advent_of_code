use std::collections::VecDeque;
use models::{Range, Row, Map};

pub fn get_structs(lines: VecDeque<String>) -> (Vec<Range>, Vec<Map>) {
    let mut lines = lines;
    let seeds = load_seeds(&mut lines);
    let maps = load_maps(&mut lines);

    (seeds, maps)
}

pub fn load_seeds(lines: &mut VecDeque<String>) -> Vec<Range> {
    let first_line = if let Some(s) = lines.pop_front() {
        let s: Vec<&str> = s
        .split(": ")
        .collect();

        let s: Vec<i64> = s[1]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

        s
    } else {
        panic!("error obtaining seed row, dequeue empty");
    };

    let mut seeds: Vec<Range> = Vec::new();

    // parsing first line
    let len = first_line.len();
    for i in 0..len {
        if i % 2 == 1 {
            let start = first_line[i - 1];
            let end = start + first_line[i] - 1; // start + length - 1
            seeds.push(Range::new(start, end));
        }
    }

    seeds
}

pub fn load_maps(lines: &mut VecDeque<String>) -> Vec<Map> {
    lines.pop_front();

    let mut state: bool = false;
    let mut maps: Vec<Map> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();

    for line in lines {
        match (state, line.is_empty()) {
            (false, _) => state = true, // { title }
            (true, false) => rows.push(Row::new(line)), // { rows }
            (true, true) => { // { empty line }
                maps.push(Map::new(rows));
                rows = Vec::new();
                state = false;
            }
        }
    }

    maps
}

pub fn merge(ranges: Vec<Vec<Range>>) -> Vec<Range> {
    let mut merged: Vec<Range> = Vec::new();
    for subrange in ranges {
        for range in subrange {
            merged.push(range);
        }
    }
    merged
}

// part2 has a submodule called models
// since the contents of this submodule are not defined inline ...
// rust searches for "src/part2/models.rs"
pub(crate) mod models;