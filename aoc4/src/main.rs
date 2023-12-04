use std::collections::HashSet;

#[derive(Debug, Clone)]
struct ScratchCard {
    index: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    won_cards: HashSet<usize>
}

impl ScratchCard {
    pub fn score(&self) -> usize {
        let mut total = 0;

        for winning_number in self.winning_numbers.iter() {
            if self.numbers.contains(winning_number) {
                total = match total {
                    0 => 1,
                    total => total * 2,
                }
            }
        }

        total
    }
}

impl From<(usize, &str)> for ScratchCard {
    fn from(value: (usize, &str)) -> Self {
        // First we remove the number of the card since we dont need it
        let (index, string) = value;
        let card_info = string.split(": ").last().unwrap();

        let mut pipe_split = card_info.split(" | ");

        let winning_numbers = number_string_to_set(pipe_split.next().unwrap());
        let numbers = number_string_to_set(pipe_split.next().unwrap());

        let winning_num_count = winning_numbers.intersection(&numbers).count();
        let won_cards = (1..=winning_num_count)
            .map(|offset| index + offset)
            .collect();

        Self {
            winning_numbers,
            numbers,
            index,
            won_cards
        }
    }
}

fn number_string_to_set(str: &str) -> HashSet<usize> {
    str.split(' ')
        .filter(|num_str| num_str != &"")
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn parse_input() -> Vec<ScratchCard> {
    std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|line| line.into())
        .collect()
}

mod part1 {
    use crate::parse_input;

    pub fn solution() {
        let total_score: usize = parse_input().iter().map(|card| card.score()).sum();

        println!("(Part 1) Scratch card score: {}", total_score);
    }
}

mod part2 {
    use std::collections::HashMap;

    use crate::{parse_input, ScratchCard};

    pub fn solution() {
        let original_cards: Vec<ScratchCard> = parse_input();
        let mut unprocessed_indexes: Vec<usize> =
            original_cards.iter().map(|card| card.index).collect();

        // Maps the card index to the amount of times we have it
        let mut processed_cards: HashMap<usize, usize> = HashMap::new();

        while let Some(index) = unprocessed_indexes.pop() {
            let card_to_process = &original_cards[index];

            match processed_cards.get(&index) {
                Some(amount) => processed_cards.insert(index, *amount + 1),
                None => processed_cards.insert(index, 1),
            };

            let new_card_indexes = &card_to_process.won_cards;
            unprocessed_indexes.extend(new_card_indexes);
        }

        let total_cards: usize = processed_cards.values().sum();
        println!("(Part 2) Total amount of cards: {}", total_cards);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
