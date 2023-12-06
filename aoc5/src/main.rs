use rayon::prelude::*;
use std::{collections::HashMap, ops::Range};

#[derive(Debug)]
struct RangeMap(HashMap<Range<usize>, Range<usize>>);

impl RangeMap {
    pub fn get(&self, value: usize) -> usize {
        match self
            .0
            .keys()
            .find(|source_range| source_range.contains(&value))
        {
            Some(source_range) => {
                let mut source_range = source_range.clone();
                let mut dest_range = self.0.get(&source_range).unwrap().clone();

                let source_lower_bound = source_range.next().unwrap();
                let dest_lower_bound = dest_range.next().unwrap();

                let offset = value - source_lower_bound;
                //dbg!(value, , lower_bound_destination, offset);
                dest_lower_bound + offset
            }
            None => value,
        }
    }
}

impl From<&str> for RangeMap {
    fn from(map_str: &str) -> Self {
        let mut map_lines = map_str.lines();
        map_lines.next().unwrap(); // Remove first line which is the name of the map

        let map: HashMap<Range<usize>, Range<usize>> = map_lines
            .map(|line| {
                let mut numbers = line
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<usize>().unwrap());
                let dest_lower_bound = numbers.next().unwrap();
                let source_lower_bound = numbers.next().unwrap();
                let range_length = numbers.next().unwrap();

                let source_range = source_lower_bound..(source_lower_bound + range_length);
                let dest_range = dest_lower_bound..(dest_lower_bound + range_length);

                (source_range, dest_range)
            })
            .collect();

        RangeMap(map)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    pub fn get_seed_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        self.humidity_to_location.get(humidity)
    }
}

impl From<String> for Almanac {
    fn from(value: String) -> Self {
        let mut sections = value.split("\n\n");

        let seeds: Vec<usize> = sections
            .next()
            .unwrap()
            .split(": ")
            .last() // Remove "seeds: "
            .unwrap()
            .split(' ')
            .map(|num_str| num_str.parse::<usize>().unwrap()) // Parse the list of numbers into vec
            .collect();

        let seed_to_soil: RangeMap = sections.next().unwrap().into();
        let soil_to_fertilizer: RangeMap = sections.next().unwrap().into();
        let fertilizer_to_water: RangeMap = sections.next().unwrap().into();
        let water_to_light: RangeMap = sections.next().unwrap().into();
        let light_to_temperature: RangeMap = sections.next().unwrap().into();
        let temperature_to_humidity: RangeMap = sections.next().unwrap().into();
        let humidity_to_location: RangeMap = sections.next().unwrap().into();

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

fn parse_input() -> Almanac {
    std::fs::read_to_string("input").unwrap().into()
}

mod part1 {
    use crate::*;
    pub fn solution() {
        let almanac = parse_input();
        let locations = almanac
            .seeds
            .iter()
            .map(|seed| almanac.get_seed_location(*seed));
        println!("(Part 1) Lowest location: {}", locations.min().unwrap());
    }
}

mod part2 {
    use crate::*;

    pub fn solution() {
        let almanac = parse_input();

        let seeds = &almanac.seeds;
        let seeds = seeds.chunks(2).par_bridge().flat_map(|chunk| {
            let lower_bound = chunk[0];
            let length = chunk[1];

            lower_bound..(lower_bound + length + 1)
        });

        let min_location = seeds
            .into_par_iter()
            .map(|seed| almanac.get_seed_location(seed))
            .min()
            .unwrap();
        println!("(Part 2) Lowest location: {}", min_location);
    }
}

fn main() {
    part1::solution();
    part2::solution();
}
