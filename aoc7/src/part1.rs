use std::collections::HashMap;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T, // Idk what T is
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn score(&self) -> usize {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand(Vec<Card>);

impl Hand {
    pub fn to_map(&self) -> HashMap<Card, usize> {
        let mut map = HashMap::new();
        
        for card in self.0.iter() {
            match map.get(card) {
                Some(count) => map.insert(card.clone(), count + 1),
                None => map.insert(card.clone(), 1),
            };
        };

        map
    }
    pub fn kind(&self) -> HandKind { 
        let cards = &self.0; 
        let map = self.to_map();

        // Five of a kind, where all five cards have the same label: AAAAA
        if cards.iter().all(|card| card == &cards[0]) {
            return HandKind::FiveOfAKind
        }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if map.values().any(|count| *count == 4) {
            return HandKind::FourOfAKind
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        if map.values().any(|count| *count == 3) && map.len() == 2 {
            return HandKind::FullHouse
        }

        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        if map.values().any(|count| *count == 3) && map.len() == 3 {
            return HandKind::ThreeOfAKind
        }

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        let mut sorted_values: Vec<usize> = map.values().map(|x| *x).collect();
        sorted_values.sort(); 
        sorted_values.reverse();
        let sorted_values = sorted_values; // Sorted in descending order
        if map.len() == 3 && sorted_values[0] == 2 && sorted_values[1] == 2 {
            return HandKind::TwoPair
        }
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        if map.len() == 4 {
            return HandKind::OnePair
        }

        if map.len() != 5 {
            panic!("Could not find a card for hand: {:#?}", self);
        }

        // High card, where all cards' labels are distinct: 23456
        return HandKind::HighCard;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind() != other.kind() {
            return Some(self.kind().cmp(&other.kind()))
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return Some(self.0.cmp(&other.0))
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Bid {
    hand: Hand,
    bid: usize,
}

impl Bid {
    pub fn calculate_winnings(&self, rank: usize) -> usize {
        self.bid * rank
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

fn parse_input() -> Vec<Bid> {
    std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| Bid {
            hand: Hand(
                line.split(' ')
                    .nth(0)
                    .unwrap()
                    .chars()
                    .map(|char| char.into())
                    .collect(),
            ),
            bid: line.split(' ').nth(1).unwrap().parse::<usize>().unwrap(),
        })
        .collect()
}

pub fn solution() {
    let mut bids = parse_input();
    bids.sort();

    let total_winnings: usize = bids.iter().enumerate().map(|(index, bid)| bid.calculate_winnings(index + 1)).sum();
    println!("(Part 1) total winnings: {total_winnings}");
}

