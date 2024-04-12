use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

#[derive(PartialEq, Clone, Debug)]
pub struct Hand {
    cards: Vec<char>,
    pub bid: u32,
    pub strength: u32,
    pub kind: Option<HandKind>,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u32) -> Self {
        let map: HashMap<char, u32> = HashMap::from([
            ('A', 12),
            ('K', 11),
            ('Q', 10),
            ('J', 9),
            ('T', 8),
            ('9', 7),
            ('8', 6),
            ('7', 5),
            ('6', 4),
            ('5', 3),
            ('4', 2),
            ('3', 1),
            ('2', 0),
        ]);

        let mut strength: u32 = 0;
        let n = cards.len();

        for i in 0..n {
            let exp = 4 - (i as u32);
            let mapped = map.get(&cards[i]).unwrap();
            strength += 13_u32.pow(exp) * mapped; // 13^(4 - i) * map[ch]
        }

        let kind = None;

        Self { cards, bid, strength, kind }

    }

    fn kind(&mut self) {
        let mut map: HashMap<char, usize> = HashMap::new();
        for ch in &self.cards {
            map.entry(*ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        let mut unique_chars: Vec<usize> = map.into_values().collect();
        unique_chars.sort_unstable();
        
        match unique_chars.len() {
            1 => self.kind = Some(HandKind::FiveKind), //     [5]
            2 => {
                if unique_chars[0] == 1 {
                    self.kind = Some(HandKind::FourKind); //  [1, 4]
                } else {
                    self.kind = Some(HandKind::FullHouse); // [2, 3]
                }
            }
            3 => {
                if unique_chars[1] == 1 {
                    self.kind = Some(HandKind::ThreeKind); // [1, 1, 3]
                } else {
                    self.kind = Some(HandKind::TwoPair); //   [1, 2, 2]
                }
            }
            4 => self.kind = Some(HandKind::OnePair), //      [1, 1, 1, 2]
            _ => self.kind = Some(HandKind::HighCard), //     [1, 1, 1, 1, 1]
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum HandKind {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn ordinal(&self) -> usize {
        match self {
            HandKind::FiveKind => 6,
            HandKind::FourKind => 5,
            HandKind::FullHouse => 4,
            HandKind::ThreeKind => 3,
            HandKind::TwoPair => 2,
            HandKind::OnePair => 1,
            HandKind::HighCard => 0,
        }
    }
}

pub fn get_hands(lines: VecDeque<String>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let contents: Vec<&str> = line.split(' ').collect();
        let (cards_str, bid_str) = (contents[0], contents[1]);
        
        let mut cards: Vec<char> = Vec::new();
        for ch in cards_str.chars() {
            cards.push(ch);
        }

        let bid = bid_str.to_owned().parse::<u32>().unwrap();

        hands.push(Hand::new(cards, bid));
    }

    for hand in &mut hands {
        hand.kind();
    }

    hands
}

pub fn compare(a: &Hand, b: &Hand) -> Ordering {
    let (aknd, bknd) = (a.kind.clone().unwrap().ordinal(), b.kind.clone().unwrap().ordinal());
    let first_ruling = aknd.cmp(&bknd);
    
    match first_ruling {
        Ordering::Equal => (),
        ord => return ord,
    }

    return a.strength.cmp(&b.strength);

}