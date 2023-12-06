
use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Race {
    pub time_ms: usize,
    pub record_distance_mm: usize,
}

impl From<Vec<String>> for Race {
    fn from(file_lines: Vec<String>) -> Self {
        let regex_times = r"^Time: ([0-9 ]+)$";
        let regex_times = Regex::new(regex_times).unwrap();
        let times_str = regex_times.captures(&file_lines[0]).unwrap().get(1).unwrap().as_str();
        let time_ms = times_str.split_ascii_whitespace().collect::<String>().parse().unwrap();

        let regex_distance = r"^Distance: ([0-9 ]+)$";
        let regex_distance = Regex::new(regex_distance).unwrap();
        let distance_str = regex_distance.captures(&file_lines[1]).unwrap().get(1).unwrap().as_str();
        let record_distance_mm = distance_str.split_ascii_whitespace().collect::<String>().parse().unwrap();

        Race {
            time_ms,
            record_distance_mm,
        }
    }
}

impl Race {
    fn number_of_ways_to_beat_the_record(&self) -> usize {
        let half_time = if self.time_ms % 2 == 0 {
            self.time_ms / 2
        } else {
            self.time_ms / 2 + 1
        };

        let mut first_record = None;
        for hold_time in 1..=half_time {
            let distance = hold_time * (self.time_ms - hold_time);
            if distance > self.record_distance_mm {
                first_record = Some(hold_time);
                break;
            }
        }

        if let Some(first_record) = first_record {
            self.time_ms + 1 - 2 * first_record
        } else {
            0
        }
    }
}

pub fn run() -> usize {
    let race = Race::from(split_input_into_lines(INPUT));

    race.number_of_ways_to_beat_the_record()
}
