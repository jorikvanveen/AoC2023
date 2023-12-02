#[derive(Debug)]
struct Subset {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Subset {
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn to_quantity_and_color(str: &str) -> Result<(usize, String), ()> {
    let mut split = str.split(' ');

    let quantity: usize = split.next().ok_or(())?.parse().map_err(|_| ())?;
    let color = split.next().ok_or(())?;

    Ok((quantity, color.into()))
}

impl TryFrom<&str> for Subset {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let colors: Result<Vec<(usize, String)>, _> =
            value.split(", ").map(to_quantity_and_color).collect();
        let colors = colors?;

        let mut subset = Subset {
            red: 0,
            green: 0,
            blue: 0,
        };

        for (quantity, color) in colors {
            match color.as_str() {
                "red" => subset.red = quantity,
                "green" => subset.green = quantity,
                "blue" => subset.blue = quantity,
                _ => return Err(()),
            }
        }

        Ok(subset)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn max_red(&self) -> usize {
        self.subsets.iter().map(|subset| subset.red).max().unwrap()
    }

    pub fn max_green(&self) -> usize {
        self.subsets
            .iter()
            .map(|subset| subset.green)
            .max()
            .unwrap()
    }

    pub fn max_blue(&self) -> usize {
        self.subsets.iter().map(|subset| subset.blue).max().unwrap()
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        // Example input:
        // Game 3: 20 green, 1 blue, 7 red; 20 green, 7 blue; 18 red, 8 green, 3 blue; 7 red, 6 blue, 11 green; 11 red, 6 blue, 16 green

        let mut colon_split = line.split(": ");

        let game_string = colon_split.next().ok_or(())?;
        let subset_string = colon_split.next().ok_or(())?;

        // Get game ID
        let id: usize = game_string
            .split(' ')
            .nth(1)
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;

        let subsets: Result<Vec<Subset>, ()> = subset_string
            .split("; ")
            .map(|color_str| color_str.try_into())
            .collect();

        let subsets = subsets?;

        Ok(Game { subsets, id })
    }
}

fn parse_input() -> Result<Vec<Game>, ()> {
    let input = std::fs::read_to_string("input").expect("Failed to read input");
    let games: Result<Vec<Game>, ()> = input.lines().map(|line| line.try_into()).collect();
    games
}

mod part1 {
    use super::*;

    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    fn is_game_possible(game: &Game) -> bool {
        game.subsets.iter().all(|subset| {
            subset.red <= MAX_RED && subset.green <= MAX_GREEN && subset.blue <= MAX_BLUE
        })
    }

    pub fn solution() {
        let games = parse_input().expect("Failed to parse games");
        let sum_of_ids: usize = games
            .into_iter()
            .filter(is_game_possible)
            .map(|game| game.id)
            .sum();
        println!("(Part 1) Sum of possible game ids: {}", sum_of_ids);
    }
}

mod part2 {
    use super::*;

    pub fn solution() {
        let games = parse_input().expect("Failed to parse games");

        let sum_of_powers: usize = games
            .into_iter()
            .map(|game| {
                Subset {
                    red: game.max_red(),
                    green: game.max_green(),
                    blue: game.max_blue(),
                }
                .power()
            })
            .sum();

        println!("(Part 2) Sum of powers: {}", sum_of_powers);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
