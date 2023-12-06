
use std::iter::zip;

use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Races {
    pub races: Vec<Race>,
}

impl From<Vec<String>> for Races {
    fn from(file_lines: Vec<String>) -> Self {
        let regex_times = r"^Time: ([0-9 ]+)$";
        let regex_times = Regex::new(regex_times).unwrap();
        let times_str = regex_times.captures(&file_lines[0]).unwrap().get(1).unwrap().as_str();

        let regex_distance = r"^Distance: ([0-9 ]+)$";
        let regex_distance = Regex::new(regex_distance).unwrap();
        let distance_str = regex_distance.captures(&file_lines[1]).unwrap().get(1).unwrap().as_str();

        let races = zip(
            times_str.split_ascii_whitespace(),
            distance_str.split_ascii_whitespace()
        )
        .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
        .map(|(time_ms, record_distance_mm)| Race { time_ms, record_distance_mm })
        .collect();

        Races {
            races
        }
    }
}

#[derive(Debug)]
struct Race {
    pub time_ms: usize,
    pub record_distance_mm: usize,
}

impl Race {
    fn number_of_ways_to_beat_the_record(&self) -> usize {
        let mut result = 0;
        for hold_time in 1..self.time_ms {
            let distance = hold_time * (self.time_ms - hold_time);
            if distance > self.record_distance_mm {
                result += 1;
            } else if result > 0 {
                break;
            }
        }
        result
    }
}

pub fn run() -> usize {
    let races = Races::from(split_input_into_lines(INPUT));

    races.races.into_iter().fold(1, |acc, race| acc * race.number_of_ways_to_beat_the_record())
}
