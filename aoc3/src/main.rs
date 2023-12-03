const ASCII_OFFSET: usize = '0' as usize;

#[derive(Debug, Clone)]
struct PositionedNumber {
    value: usize,
    row: usize,
    position: (usize, usize), // (col_start, col_end)
}

impl PositionedNumber {
    pub fn get_surrounding_positions(&self) -> Vec<(usize, usize)> {
        // We need to check the following positions:

        // xxxxx
        // x153x
        // xxxxx

        // On row_idx - 1: (col_start-1..=col_end+1)
        // On row_idx: col_start-1 and col_end+1
        // On row_idx + 1: (col_start-1..=col_end+1)

        // If any of these positions contain a symbol, this is a valid part number
        let row_idx = self.row;
        let (col_start_check, col_end_check) = match self.position {
            (0, col_end_check) => (0, col_end_check + 1), // Prevent 0 - 1 on a usize
            (col_start_check, col_end_check) => (col_start_check - 1, col_end_check + 1),
        };

        let mut positions_to_check: Vec<(usize, usize)> = vec![];

        if row_idx > 0 {
            positions_to_check
                .extend((col_start_check..=col_end_check).map(|col_idx| (row_idx - 1, col_idx)));
        }

        positions_to_check.push((row_idx, col_start_check));
        positions_to_check.push((row_idx, col_end_check));
        positions_to_check
            .extend((col_start_check..=col_end_check).map(|col_idx| (row_idx + 1, col_idx)));

        positions_to_check
    }
}

#[derive(Debug)]
enum SchematicValue {
    Period,
    Symbol(char),
    Digit(usize),
}

impl From<char> for SchematicValue {
    fn from(char: char) -> Self {
        match char {
            '0'..='9' => SchematicValue::Digit(char as usize - ASCII_OFFSET),
            '.' => SchematicValue::Period,
            char => SchematicValue::Symbol(char),
        }
    }
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Vec<SchematicValue>>,
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        Self {
            rows: value
                .lines()
                .map(|line| line.chars().map(|char| char.into()).collect())
                .collect(),
        }
    }
}

impl Schematic {
    pub fn get_row(&self, row_idx: usize) -> Option<&Vec<SchematicValue>> {
        self.rows.get(row_idx)
    }

    pub fn get_value(&self, row_idx: usize, col_idx: usize) -> Option<&SchematicValue> {
        self.get_row(row_idx).and_then(|row| row.get(col_idx))
    }

    pub fn get_all_numbers(&self) -> Vec<PositionedNumber> {
        let mut all_numbers: Vec<PositionedNumber> = vec![];

        for (row_idx, row) in self.rows.iter().enumerate() {
            // We need to assemble the numbers from the digits in the schematic
            // We also need to worry about where in the schematic these numbers start and end
            // since we want to know if there is a symbol around any given number.
            let digits = row
                .iter()
                .enumerate()
                .filter_map(|(col_idx, value)| match value {
                    SchematicValue::Digit(digit) => Some((col_idx, digit)),
                    _ => None,
                });

            let mut digit_collections: Vec<Vec<(usize, usize)>> = vec![];

            for (col_idx, digit) in digits {
                let last_number = match digit_collections.last_mut() {
                    Some(last_number) => last_number,
                    None => {
                        let new_number = vec![];
                        digit_collections.push(new_number);
                        digit_collections.last_mut().unwrap()
                    }
                };
                let last_digit = last_number.last();

                match last_digit {
                    Some((last_index, _)) => {
                        if col_idx == last_index + 1 {
                            last_number.push((col_idx, *digit))
                        } else {
                            let new_number = vec![(col_idx, *digit)];
                            digit_collections.push(new_number);
                        }
                    }
                    None => last_number.push((col_idx, *digit)),
                }
            }

            // Now that we have all the groups of digits and their positions, we can convert them
            // to numbers

            let numbers: Vec<PositionedNumber> = digit_collections
                .into_iter()
                .map(|collection| PositionedNumber {
                    // If we have digits 1, 5 and 3. We can reverse and use the indexes:
                    // Index: 0 1 2
                    // Value: 3 5 1
                    // 3 * 10^0 + 5 * 10^1 + 1 * 10^2 = 153
                    value: collection
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(index, (_, digit))| digit * (10_usize.pow(index as u32)))
                        .sum(),
                    row: row_idx,
                    position: (
                        *collection.iter().map(|(index, _)| index).min().unwrap(),
                        *collection.iter().map(|(index, _)| index).max().unwrap(),
                    ),
                })
                .collect();

            all_numbers.extend(numbers.into_iter());
        }

        all_numbers
    }
}

fn parse_input() -> Schematic {
    std::fs::read_to_string("input").unwrap().as_str().into()
}

mod part1 {
    use crate::*;

    pub fn solution() {
        let schematic = parse_input();

        let all_numbers = schematic.get_all_numbers();

        let sum_of_part_numbers: usize = all_numbers
            .into_iter()
            .filter(|number| {
                let positions_to_check = number.get_surrounding_positions();

                positions_to_check.into_iter().any(|(row_idx, col_idx)| {
                    matches!(
                        schematic.get_value(row_idx, col_idx),
                        Some(SchematicValue::Symbol(_))
                    )
                })
            })
            .map(|number| number.value)
            .sum();

        println!("(Part 1) Sum of all part numbers: {}", sum_of_part_numbers);
    }
}

mod part2 {
    use std::collections::HashMap;

    use crate::*;

    pub fn solution() {
        let schematic = parse_input();

        let all_numbers = schematic.get_all_numbers();

        // We associate each position with numbers belonging to a given gear
        let mut possible_gears: HashMap<(usize, usize), Vec<PositionedNumber>> = HashMap::new();

        for number in all_numbers {
            let positions_to_check = number.get_surrounding_positions();

            positions_to_check
                .into_iter()
                .for_each(|(row_idx, col_idx)| {
                    if matches!(
                        schematic.get_value(row_idx, col_idx),
                        Some(SchematicValue::Symbol('*'))
                    ) {
                        match possible_gears.get_mut(&(row_idx, col_idx)) {
                            Some(entry) => entry.push(number.clone()),
                            None => {
                                possible_gears.insert((row_idx, col_idx), vec![number.clone()]);
                            }
                        };
                    }
                })
        }

        let sum: usize = possible_gears
            .values()
            .filter(|numbers| numbers.len() >= 2)
            .map(|numbers| numbers.iter().map(|number| number.value).product::<usize>())
            .sum();

        println!("(Part 2) Sum of gear ratios: {}", sum);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
