#[derive(Debug, Clone)]
struct History(Vec<isize>);

impl History {
    pub fn derivative(&self) -> History {
        History(
            self.0
                .windows(2) // Split readings into windows of size 2
                .map(|window| window[1] - window[0]) // Get the difference in all of the windows
                .collect(), // Put the resulting values in a Vec
        )
    }

    pub fn get_all_derivatives(&self) -> Vec<History> {
        let mut derivatives: Vec<History> = vec![self.clone()];

        loop {
            let derivative = match derivatives.last() {
                Some(derivative) => derivative.derivative(),
                None => self.derivative(),
            };

            let done = derivative.0.iter().all(|reading| reading == &0);
            derivatives.push(derivative);

            if done {
                break;
            }
        }

        derivatives
    }

    pub fn extrapolate_next(&self) -> isize {
        let mut derivatives = self.get_all_derivatives();
        let mut extrapolated_value = 0;

        while let Some(derivative) = derivatives.pop() {
            extrapolated_value += derivative.0.last().unwrap();
        }

        extrapolated_value
    }

    pub fn extrapolate_previous(&self) -> isize {
        let mut derivatives = self.get_all_derivatives();
        let mut extrapolated_value = 0;

        while let Some(derivative) = derivatives.pop() {
            extrapolated_value = derivative.0.first().unwrap() - extrapolated_value;
        }

        extrapolated_value
    }
}

impl From<&str> for History {
    fn from(line: &str) -> Self {
        Self(
            line.split_ascii_whitespace()
                .map(|number| number.parse::<isize>().unwrap())
                .collect(),
        )
    }
}

fn parse_input() -> Vec<History> {
    std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.into())
        .collect()
}

mod part1 {
    use crate::*;
    pub fn solution() {
        let histories = parse_input();
        let sum: isize = histories
            .into_iter()
            .map(|history| history.extrapolate_next())
            .sum();

        println!("(Part 1) Sum of extrapolated values: {}", sum);
    }
}

mod part2 {
    use crate::*;
    pub fn solution() {
        let histories = parse_input();
        let sum: isize = histories
            .into_iter()
            .map(|history| history.extrapolate_previous())
            .sum();

        println!("(Part 2) Sum of extrapolated values: {}", sum);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
