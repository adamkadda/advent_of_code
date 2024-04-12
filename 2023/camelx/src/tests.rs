mod kind_tests {
    use crate::part2::{Hand, HandKind};
    
    #[test]
    fn test_high_card_normal() {
        let cards: Vec<char> = vec!['2', '3', '4', '5', '6'];
        let bid: u32 = 1;

        let mut hand = Hand::new(cards, bid);
        hand.kind();
        
        assert_eq!(hand.kind.unwrap(), HandKind::HighCard);
    }

    #[test]
    fn test_one_pair_normal() {
        let cards: Vec<char> = vec!['2', '2', '4', '5', '6'];
        let bid: u32 = 1;

        let mut hand = Hand::new(cards, bid);
        hand.kind();
        
        assert_eq!(hand.kind.unwrap(), HandKind::OnePair);
    }

    #[test]
    fn test_one_pair_1joker() {
        let cards: Vec<char> = vec!['J', '3', '4', '5', '6'];
        let bid: u32 = 1;

        let mut hand = Hand::new(cards, bid);
        hand.kind();
        
        assert_eq!(hand.kind.unwrap(), HandKind::OnePair);
    }

    #[test]
    fn test_two_pair_1joker() {
        let cards: Vec<char> = vec!['2', '2', '3', 'J', '4'];
        let bid: u32 = 1;

        let mut hand = Hand::new(cards, bid);
        hand.kind();
        
        assert_eq!(hand.kind.unwrap(), HandKind::TwoPair);
    }

    #[test]
    fn test_two_pair_2joker() {
        let cards: Vec<char> = vec!['2', 'J', 'J
        ', '5', '6'];
        let bid: u32 = 1;

        let mut hand = Hand::new(cards, bid);
        hand.kind();
        
        assert_eq!(hand.kind.unwrap(), HandKind::OnePair);
    }
}