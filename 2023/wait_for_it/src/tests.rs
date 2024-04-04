mod left_tests {
    use crate::parts::Race;

    #[test]
    fn test_left_6() {
        let race = Race::new(6.0, 7.0);
        assert_eq!(race.left(), 1.0);
    }

    #[test]
    fn test_left_7() {
        let race = Race::new(7.0, 7.0);
        assert_eq!(race.left(), 1.0);
    }

    #[test]
    fn test_left_8() {
        let race = Race::new(8.0, 7.0);
        assert_eq!(race.left(), 1.0);
    }

    #[test]
    fn test_left_9() {
        let race = Race::new(9.0, 7.0);
        assert_eq!(race.left(), 1.0);
    }

    #[test]
    fn test_left_10() {
        let race = Race::new(10.0, 7.0);
        assert_eq!(race.left(), 2.0);
    }
}

mod ways_tests {
    use crate::parts::Race;

    #[test]
    fn test_ways_odd() {
        let race = Race::new(9.0, 7.0);
        let correct: u64 = 4;
        assert_eq!(race.ways_to_win(), correct);
    }

    #[test]
    fn test_ways_even() {
        let race = Race::new(200.0, 30.0);
        let correct: u64 = 9;
        assert_eq!(race.ways_to_win(), correct);
    }
}