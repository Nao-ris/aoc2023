
use std::collections::HashMap;

use super::utils::split_input_into_lines;

use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Unknown color: {}", s),
        }
    }
}

#[derive(Debug)]
struct Game {
    pub id: u16,
    pub sets: Vec<HashMap<Color, u16>>,
}

impl From<String> for Game {
    fn from(s: String) -> Self {
        let regex_game = r"^Game (?P<id>\d+): ([0-9a-z ,]+)(?:; (?:[0-9a-z ,]+))*$";
        let regex_game = Regex::new(regex_game).unwrap();
    
        let regex_next_sets = r"; ([0-9a-z ,]+)";
        let regex_next_sets = Regex::new(regex_next_sets).unwrap();

        fn str_to_set(s: &str) -> HashMap<Color, u16> {
            let regex_color = r"(\d+) ([a-z]+)";
            let regex_color = Regex::new(regex_color).unwrap();

            let mut result = HashMap::new();
            regex_color
                .captures_iter(s)
                .map(|c| c.extract())
                .for_each(|(_, [color_number, color])| {
                    result.insert(Color::from(color), color_number.parse().unwrap());
                });
            result
        }

        if let Some(cap) = regex_game.captures(&s) {
            let id: u16 = cap.get(1).unwrap().as_str().parse().unwrap();

            let mut sets = vec![
                str_to_set(cap.get(2).unwrap().as_str())
            ];

            for s in regex_next_sets.captures_iter(&s) {
                sets.push(str_to_set(s.get(0).unwrap().as_str()));
            }

            Game {
                id: id,
                sets: sets,
            }
        } else {
            Game {
                id: 0,
                sets: vec![],
            }
        }
    }
}

impl Game {
    fn compatible_with(&self, max_cubes: &HashMap<Color, u16>) -> bool {
        for (max_key, max_value) in max_cubes {
            for set in self.sets.iter() {
                if let Some(value) = set.get(max_key) {
                    if value > max_value {
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub fn run() -> usize {
    let max_cubes = HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]);

    split_input_into_lines(INPUT)
        .into_iter()
        .map(Game::from)
        .fold(0, |acc, game| {
            if game.compatible_with(&max_cubes) {
                acc + (game.id as usize)
            } else {
                acc
            }
        })
}
