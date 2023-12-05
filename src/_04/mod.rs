
use std::{collections::HashMap, cmp::max};

use super::utils::split_input_into_lines;

use regex::Regex;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    pub _id: u16,
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
                _id: id,
                sets: sets,
            }
        } else {
            Game {
                _id: 0,
                sets: vec![],
            }
        }
    }
}

impl Game {
    fn power(&self) -> usize {
        let mut min_cubes = HashMap::from([
            (Color::Red, 0 as usize),
            (Color::Green, 0 as usize),
            (Color::Blue, 0 as usize),
        ]);

        for set in self.sets.iter() {
            for (key, value) in set {
                min_cubes.insert(key.clone(), max(*min_cubes.get(key).unwrap(), *value as usize));
            }
        }

        min_cubes.get(&Color::Red).unwrap() * min_cubes.get(&Color::Green).unwrap() * min_cubes.get(&Color::Blue).unwrap()
    }
}

pub fn run() -> usize {
    split_input_into_lines(INPUT)
        .into_iter()
        .map(Game::from)
        .fold(0, |acc, game| {
            acc + game.power()
        })
}
