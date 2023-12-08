use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Left,
    Right
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction")
        }
    }
}

#[derive(Debug)]
struct Network(HashMap<String, (String, String)>);

impl Network {
    pub fn left(&self, node: &String) -> &String {
        &self.0.get(node).unwrap().0
    }

    pub fn right(&self, node: &String) -> &String {
        &self.0.get(node).unwrap().1
    }
}

fn parse_input() -> (Vec<Instruction>, String, Network) {
    let input = std::fs::read_to_string("input").unwrap();
    let mut split = input.split("\n\n");

    let instructions_str = split.next().unwrap();
    let network_str = split.next().unwrap();

    let nodes_vec: Vec<(String, (String, String))> = network_str.lines().map(|line| {
        let mut equals_split = line.split(" = ");
        let key = equals_split.next().unwrap().to_owned();
        
        let mut values = equals_split.next().unwrap().split(", ");
        let left_value = values.next().unwrap().replace("(", "");
        let right_value = values.next().unwrap().replace(")", "");
        
        (key, (left_value, right_value))
    }).collect();

    let first_node = nodes_vec.first().unwrap().0.clone();

    (instructions_str.chars().map(|char| char.into()).collect(), first_node, Network(nodes_vec.into_iter().collect()))
}

mod part1 {
    use crate::*;

    pub fn solution() {
        let (instructions, first_node, network) = parse_input();
        
        let mut current_node = "AAA".to_owned();
        let mut instruction_idx = 0;

        while current_node != "ZZZ" {
            let instruction = &instructions[instruction_idx % instructions.len()]; 

            current_node = match instruction {
                Instruction::Left => network.left(&current_node),
                Instruction::Right => network.right(&current_node),
            }.clone();

            instruction_idx += 1;

        }

        println!("(Part 1) Steps required: {instruction_idx}");
    }
}
mod part2 {
    use crate::*;

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    fn lcm_many(factors: Vec<usize>) -> usize {
        let mut factors = factors.into_iter();
        let mut result = factors.next().unwrap();

        while let Some(factor) = factors.next() {
            result = lcm(result, factor);
        }

        result
    }

    pub fn solution() {
        let (instructions, first_node, network) = parse_input();
        
        let starting_nodes = network.0.keys().filter(|key| key.chars().last() == Some('A'));
        let mut distances: Vec<usize> = vec![];

        for start in starting_nodes {
            let mut current_node = start.clone();
            let mut instruction_idx = 0;

            while current_node.chars().last() != Some('Z') {
                let instruction = &instructions[instruction_idx % instructions.len()]; 

                current_node = match instruction {
                    Instruction::Left => network.left(&current_node),
                    Instruction::Right => network.right(&current_node),
                }.clone();

                instruction_idx += 1;
            }

            distances.push(instruction_idx);
        }

        let lcm: usize = lcm_many(distances);
        println!("(Part 2) LCM: {}", lcm);

    }
}

fn main() {
    part1::solution();
    part2::solution();
}
