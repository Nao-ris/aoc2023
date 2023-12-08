
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(char: char) -> Direction {
        match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug, Eq)]
struct Node {
    id: String,
    mappings: HashMap<Direction, String>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashSet<Node>,
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let regex_directions = r"^([RL]+)$";
        let regex_directions = Regex::new(regex_directions).unwrap();
        let directions_str = regex_directions.captures(&lines[0]).unwrap().get(1).unwrap().as_str();

        let regex_node = r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$";
        let regex_node = Regex::new(regex_node).unwrap();
        let mut nodes = HashSet::new();
        for i in 2..lines.len() {
            let node_id = regex_node.captures(&lines[i]).unwrap().get(1).unwrap().as_str();
            let left_node_id = regex_node.captures(&lines[i]).unwrap().get(2).unwrap().as_str();
            let right_node_id = regex_node.captures(&lines[i]).unwrap().get(3).unwrap().as_str();
            nodes.insert(Node {
                id: node_id.to_string(),
                mappings: HashMap::from([
                    (Direction::Left, left_node_id.to_string()),
                    (Direction::Right, right_node_id.to_string()),
                ]),
            });
        }

        Map {
            instructions: directions_str.chars().map(Direction::from).collect(),
            nodes,
        }
    }
}

impl Map {
    pub fn step_until_node(&self, node: String) -> usize {
        let mut number_of_steps = 0;

        let mut current_node = self.nodes.get(&Node { id: "AAA".to_string(), mappings: HashMap::new() }).unwrap();
        while current_node.id != node {
            let instruction = &self.instructions[number_of_steps % self.instructions.len()];
            let next_node_id = current_node.mappings.get(instruction).unwrap();

            current_node = self.nodes.get(&Node { id: next_node_id.clone(), mappings: HashMap::new() }).unwrap();
            number_of_steps += 1;
        }

        number_of_steps
    }
}

pub fn run() -> usize {
    let map = Map::from(split_input_into_lines(INPUT));
    map.step_until_node("ZZZ".to_string())
}
