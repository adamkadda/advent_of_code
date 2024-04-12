use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

#[derive(PartialEq, Clone, Debug)]
pub struct Hand {
    cards: Vec<char>,
    pub bid: u32,
    strength: u32,
    pub kind: Option<HandKind>,
    jokers: usize,
}

impl Hand {
    pub fn new(cards: Vec<char>, bid: u32) -> Self {
        let map: HashMap<char, u32> = HashMap::from([
            ('A', 12),
            ('K', 11),
            ('Q', 10),
            ('T', 9),
            ('9', 8),
            ('8', 7),
            ('7', 6),
            ('6', 5),
            ('5', 4),
            ('4', 3),
            ('3', 2),
            ('2', 1),
            ('J', 0),
        ]);

        let mut strength: u32 = 0;
        let n = cards.len();

        for i in 0..n {
            let exp = 4 - (i as u32);
            let mapped = map.get(&cards[i]).unwrap();
            strength += 13_u32.pow(exp) * mapped; // 13^(4 - i) * map[ch]
        }

        let kind = None;

        let mut jokers: usize = 0;
        let j = 'J';
        for card in &cards {
            if card == &j { jokers += 1 }
        }

        Self { cards, bid, strength, kind, jokers }

    }

    pub fn kind(&mut self) {
        let mut map: HashMap<char, usize> = HashMap::new();
        for ch in &self.cards {
            map.entry(*ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        let mut unique_chars: Vec<usize> = map.into_values().collect();
        unique_chars.sort_unstable();

        // find real kind
        // new_kind = real_kind += jokers

        self.kind = match unique_chars.len() {
            1 => Some(HandKind::FiveKind),
            2 => {
                if unique_chars[0] == 1 {
                    match self.jokers {
                        0 => Some(HandKind::FourKind),
                        1 | 4 => Some(HandKind::FiveKind),
                        _ => panic!("Four-kind -> invalid jokers: {}", self.jokers),
                    }
                } else {
                    match self.jokers {
                        0 => Some(HandKind::FullHouse),
                        2 | 3 => Some(HandKind::FiveKind),
                        _ => panic!("Full-house -> invalid jokers: {}", self.jokers),
                    }
                }
            }
            3 => {
                if unique_chars[1] == 1 {
                    match self.jokers {
                        0 => Some(HandKind::ThreeKind),
                        1 | 3 => Some(HandKind::FourKind),
                        _ => panic!("Three-kind -> invalid jokers: {}", self.jokers),
                    }
                } else {
                    match self.jokers {
                        0 => Some(HandKind::TwoPair),
                        1 => Some(HandKind::FullHouse),
                        2 => Some(HandKind::FourKind),
                        _ => panic!("Two-pair -> invalid jokers: {}", self.jokers),
                    }
                }
            }
            4 => match self.jokers {
                0 => Some(HandKind::OnePair),
                1 | 2 => Some(HandKind::ThreeKind),
                _ => panic!("One-pair -> invalid jokers: {}", self.jokers),
            }
            5 => match self.jokers {
                0 => Some(HandKind::HighCard),
                1 => Some(HandKind::OnePair),
                _ => panic!("High-card -> invalid jokers: {}", self.jokers),
            },
            _ => panic!("invalid unique_chars.len(): {}", unique_chars.len()),
        };

        
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