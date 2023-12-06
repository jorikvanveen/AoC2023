#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse_input() -> Vec<Race> {
    let input = std::fs::read_to_string("input").unwrap();

    let mut numbers = input.lines().map(|line| {
        line.split(": ")
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<usize>().unwrap())
    });

    let times = numbers.next().unwrap();
    let distances = numbers.next().unwrap();

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

mod part1 {
    use crate::*;

    pub fn solution() {
        let product_of_ways: usize = parse_input()
            .into_iter()
            .map(|race| {
                // All the possible lengths of time you can press the button for
                let button_press_times = 0..=race.time;
                button_press_times
                    .filter(|press_time| {
                        let speed = press_time;
                        let travel_time = race.time - press_time;
                        let distance_travelled = speed * travel_time;
                        distance_travelled > race.distance
                    })
                    .count()
            })
            .product();

        println!("(Part 1) Product of ways: {product_of_ways}");
    }
}

mod part2 {
    use crate::*;
    pub fn solution() {
        let original_input = parse_input();

        let time = original_input.iter().fold(String::from(""), |acc, race| {
            acc + race.time.to_string().as_str()
        }).parse::<usize>().unwrap();
        let distance = original_input.iter().fold(String::from(""), |acc, race| {
            acc + race.distance.to_string().as_str()
        }).parse::<usize>().unwrap();

        let button_press_times = 0..=time;
        let number_of_ways = button_press_times
                .filter(|press_time| {
                    let speed = press_time;
                    let travel_time = time - press_time;
                    let distance_travelled = speed * travel_time;
                    distance_travelled > distance
                })
                .count();

        println!("(Part 2) Number of ways: {number_of_ways}"); 

    }
}

fn main() {
    part1::solution();
    part2::solution();
}
