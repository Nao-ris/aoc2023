
use std::cmp::min;

use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Almanac {
    pub seeds: Vec<SeedsGroup>,
    pub maps: Vec<Map>,
}

#[derive(Debug)]
struct SeedsGroup {
    pub start_range: usize,
    pub range_length: usize,
}

#[derive(Debug)]
struct Map {
    pub _source_category: String,
    pub _destination_category: String,
    pub mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    pub source_range_start: usize,
    pub destination_range_start: usize,
    pub range_length: usize,
}

impl From<Vec<String>> for Almanac {
    fn from(file_lines: Vec<String>) -> Self {
        // Seeds
        let regex_seeds = r"^seeds: ([0-9 ]+)$";
        let regex_seeds = Regex::new(regex_seeds).unwrap();

        let seeds_str = regex_seeds.captures(&file_lines[0]).unwrap().get(1).unwrap().as_str();
        let seeds = seeds_str.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
        
        let seeds_start_range = seeds.iter().enumerate().filter(|(index, _)| index % 2 == 0).map(|(_, value)| value);
        let seeds_range_length = seeds.iter().enumerate().filter(|(index, _)| index % 2 == 1).map(|(_, value)| value);
        let seeds = seeds_start_range.zip(seeds_range_length).map(|(start, length)| {
            SeedsGroup {
                start_range: *start,
                range_length: *length,
            }
        }).collect();

        // Maps
        let maps = file_lines[2..file_lines.len()]
            .split(|line| line.is_empty())
            .map(Map::from)
            .collect();

        Almanac {
            seeds,
            maps,
        }
    }
}

impl From<&[String]> for Map {
    fn from(file_lines: &[String]) -> Self {
        // Source & Destinations
        let regex_names = r"^([a-z]+)-to-([a-z]+) map:$";
        let regex_names = Regex::new(regex_names).unwrap();
        let cap: regex::Captures<'_> = regex_names.captures(&file_lines[0]).unwrap();

        let source_category = cap.get(1).unwrap().as_str().to_string();
        let destination_category = cap.get(2).unwrap().as_str().to_string();

        // Mappings
        let mut mappings: Vec<Mapping> = file_lines[1..file_lines.len()]
            .into_iter()
            .map(Mapping::from)
            .collect();
        mappings.sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));

        Map {
            _source_category: source_category,
            _destination_category: destination_category,
            mappings,
        }
    }
}

impl From<&String> for Mapping {
    fn from(line: &String) -> Self {
        // Source & Destinations
        let regex_mapping = r"^([0-9]+) ([0-9]+) ([0-9]+)$";
        let regex_mapping = Regex::new(regex_mapping).unwrap();
        let cap = regex_mapping.captures(&line).unwrap();

        let destination_range_start = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let source_range_start = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let range_length = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

        Mapping {
            source_range_start,
            destination_range_start,
            range_length,
        }
    }
}

impl Almanac {
    fn map(&self, mut input: usize) -> usize {
        for map in self.maps.iter() {
            input = map.map(input)
        }
        input
    }
}

impl Map {
    fn map(&self, input: usize) -> usize {
        for mapping in self.mappings.iter() {
            if input < mapping.source_range_start {
                return input;
            } else if input < mapping.source_range_start + mapping.range_length {
                return mapping.destination_range_start + input - mapping.source_range_start;
            }
        }

        input
    }
}

pub fn run() -> usize {
    let almanac = Almanac::from(split_input_into_lines(INPUT));

    let mut result = usize::MAX;
    almanac
        .seeds
        .iter()
        .for_each(|seed| {
            for i in 0..seed.range_length {
                let position = almanac.map(seed.start_range + i);
                result = min(result, position);
            }
        });
    result
}
