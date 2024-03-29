mod true_true {
    use crate::part2::models::*;

    #[test]
    fn exact_match() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(40, 50);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(50, 60));

        assert_eq!(output, correct);
    }

    #[test]
    fn smaller() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(41, 49);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(51, 59));

        assert_eq!(output, correct);
    }
}

mod true_false {
    use crate::part2::models::*;

    #[test]
    fn protruding_end() { // start == row.start && end > row.end
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(40, 60);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(50, 60));
        correct.push(Range::new(51, 60));

        assert_eq!(output, correct);
    }

    #[test]
    fn end_overlap() { // start == row.end && end > row.end
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(50, 60);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(60, 60));
        correct.push(Range::new(51, 60));

        assert_eq!(output, correct);
    }
}

mod false_true {
    use crate::part2::models::*;

    #[test]
    fn protruding_start() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(30, 50);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(30, 39)); // protruding section before start
        correct.push(Range::new(50, 60)); // overlapping area

        assert_eq!(output, correct);
    }

    #[test]
    fn start_overlap() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(30, 40);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(30, 39)); // protruding pre-start
        correct.push(Range::new(50, 50)); // overlapping area (end == row.start)

        assert_eq!(output, correct);
    }
}

mod false_false {
    use crate::part2::models::*;

    #[test]
    fn lower_miss() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(20, 30);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(20, 30));

        assert_eq!(output, correct);
    }

    #[test]
    fn higher_miss() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(60, 70);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(60, 70));

        assert_eq!(output, correct);
    }

    #[test]
    fn bigger() {
        let mut rows: Vec<Row> = Vec::new();
        rows.push(Row { dest: 50, src: 40, range: 11 });
        let map = Map::new(rows);

        let input = Range::new(30, 60);
        let output = map.process(&input);

        let mut correct: Vec<Range> = Vec::new();
        correct.push(Range::new(30, 39)); // protruding pre-start
        correct.push(Range::new(50, 60)); // overlapping area
        correct.push(Range::new(51, 60)); // protruding post-end

        assert_eq!(output, correct);
    }
}