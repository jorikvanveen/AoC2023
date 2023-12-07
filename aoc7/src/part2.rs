use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Hash)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T, // Idk what T is
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn score(&self) -> usize {
        match self {
            Card::Jack => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Card::Jack,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord, Clone, Hash)]
struct Hand(Vec<Card>);

impl Hand {
    pub fn to_map(&self) -> HashMap<Card, usize> {
        let mut map = HashMap::new();

        for card in self.0.iter() {
            match map.get(card) {
                Some(count) => map.insert(card.clone(), count + 1),
                None => map.insert(card.clone(), 1),
            };
        }

        map
    }

    pub fn permutations(&self) -> HashSet<Hand> {
        let mut permutations = HashSet::new();
        permutations.insert(self.clone());

        let j_count = self.0.iter().filter(|card| card == &&Card::Jack).count();

        if j_count == 0 {
            return permutations;
        }

        let j_index = self.0.iter().position(|card| card == &Card::Jack).unwrap();

        // Get the position of the first jack
        // for each possible value of the first jack: store a copy of this hand with the first jack
        // substituted with the possible value. Also store the permutations of this copy.

        let substitutions = vec![
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::T, // Idk what T is
            Card::Queen,
            Card::King,
            Card::Ace,
        ];

        for substitution in substitutions {
            let mut permutation = self.clone();
            permutation.0[j_index] = substitution;

            if j_count > 1 {
                permutations.extend(permutation.permutations());
            }

            permutations.insert(permutation);
        }

        permutations
    }

    pub fn kind(&self) -> HandKind {
        // Replace all the J's in the cloned hand with whichever letter is most common
        let cards = &self.0;
        let map = self.to_map();

        // Five of a kind, where all five cards have the same label: AAAAA
        if cards.iter().all(|card| card == &cards[0]) {
            return HandKind::FiveOfAKind;
        }

        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if map.values().any(|count| *count == 4) {
            return HandKind::FourOfAKind;
        }

        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        if map.values().any(|count| *count == 3) && map.len() == 2 {
            return HandKind::FullHouse;
        }

        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        if map.values().any(|count| *count == 3) && map.len() == 3 {
            return HandKind::ThreeOfAKind;
        }

        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        let mut sorted_values: Vec<usize> = map.values().map(|x| *x).collect();
        sorted_values.sort();
        sorted_values.reverse();
        let sorted_values = sorted_values; // Sorted in descending order
        if map.len() == 3 && sorted_values[0] == 2 && sorted_values[1] == 2 {
            return HandKind::TwoPair;
        }
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        if map.len() == 4 {
            return HandKind::OnePair;
        }

        if map.len() != 5 {
            panic!("Could not find a card for hand: {:#?}", self);
        }

        // High card, where all cards' labels are distinct: 23456
        return HandKind::HighCard;
    }

    pub fn possible_kinds(&self) -> HashSet<HandKind> {
        let mut kinds = HashSet::new();

        for permutation in self.permutations() {
            kinds.insert(permutation.kind());
        }

        kinds
    }

    pub fn best_possible_kind(&self) -> HandKind {
        self.possible_kinds().into_iter().max().unwrap()
    }

    pub fn best_permutation(&self) -> Hand {
        self.permutations().into_iter().max().unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let kind_self = self.best_possible_kind();
        let kind_other = other.best_possible_kind();

        if kind_self != kind_other {
            return Some(kind_self.cmp(&kind_other));
        }

        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                return Some(self.0.cmp(&other.0));
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
    //bids.iter_mut().for_each(|bid| bid.hand = bid.hand.best_permutation());
    bids.sort();

    let total_winnings: usize = bids
        .iter()
        .enumerate()
        .map(|(index, bid)| bid.calculate_winnings(index + 1))
        .sum();

    println!("(Part 2) total winnings: {total_winnings}");
}
