/*
 --- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---
Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/

const ASCII_OFFSET: usize = '0' as usize;

fn is_digit(char: &char) -> bool {
    *char >= '0' && *char <= '9'
}

mod part1 {
    use super::*;

    fn get_calibration_value(line: &str) -> usize {
        let first = line.chars().find(is_digit).expect("No digits found") as usize - ASCII_OFFSET;
        let last = line.chars().rfind(is_digit).expect("No digits found") as usize - ASCII_OFFSET;
        first * 10 + last
    }

    pub fn solution() {
        let input = std::fs::read_to_string("input").expect("Failed to read input, does it exist?");
        let sum: usize = input.lines().map(get_calibration_value).sum();
        println!("(Part 1) Sum of all calibration values: {}", sum);
    }
}

mod part2 {
    const DIGIT_STRINGS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    pub fn digit_string_to_usize(digit_str: &str) -> Option<usize> {
        DIGIT_STRINGS
            .iter()
            .position(|possible_digit_str| &digit_str == possible_digit_str)
            .map(|pos| pos % 9 + 1)
    }

    pub fn get_calibration_value(line: &str) -> usize {
        // The format of this tuple is: (The string that was found, position of first occurrence,
        // position of last occurrence)
        let search_results: Vec<(String, usize, usize)> = DIGIT_STRINGS
            .iter()
            .map(|digit_string| {
                (
                    digit_string,
                    line.find(digit_string),
                    line.rfind(digit_string),
                )
            })
            .filter(|(_, first_position, last_position)| {
                first_position.is_some() && last_position.is_some()
            })
            .map(|(digit_string, first_position, last_position)| {
                (
                    digit_string.to_string(),
                    first_position.unwrap(),
                    last_position.unwrap(),
                )
            })
            .collect();

        let first_result = search_results
            .iter()
            .min_by_key(|(_, first_position, _)| first_position)
            .expect("Could not find any digits");

        let last_result = search_results
            .iter()
            .max_by_key(|(_, _, last_position)| last_position)
            .expect("Could not find any digits");

        let first_digit =
            digit_string_to_usize(&first_result.0).expect("Found invalid first digit (how?)");
        let last_digit =
            digit_string_to_usize(&last_result.0).expect("Found invalid last digit (how?)");

        first_digit * 10 + last_digit
    }

    pub fn solution() {
        let input = std::fs::read_to_string("input").expect("Failed to read input, does it exist?");
        let sum: usize = input.lines().map(get_calibration_value).sum();
        println!("(Part 2) Sum of all calibration values: {}", sum);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
