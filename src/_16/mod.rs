
use std::collections::HashMap;
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
    nodes: HashMap<String, Node>,
}

impl From<Vec<String>> for Map {
    fn from(lines: Vec<String>) -> Self {
        let regex_directions = r"^([RL]+)$";
        let regex_directions = Regex::new(regex_directions).unwrap();
        let directions_str = regex_directions.captures(&lines[0]).unwrap().get(1).unwrap().as_str();

        let regex_node = r"^([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)$";
        let regex_node = Regex::new(regex_node).unwrap();
        let mut nodes = HashMap::new();
        for i in 2..lines.len() {
            let node_id = regex_node.captures(&lines[i]).unwrap().get(1).unwrap().as_str();
            let left_node_id = regex_node.captures(&lines[i]).unwrap().get(2).unwrap().as_str();
            let right_node_id = regex_node.captures(&lines[i]).unwrap().get(3).unwrap().as_str();
            nodes.insert(node_id.to_string(), Node {
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
    pub fn steps_until_end(&self) -> usize {
        let mut current_nodes: Vec<&Node> = self.nodes.values().filter(|n| n.id.ends_with('A')).collect();

        let number_of_steps: Vec<usize> = current_nodes.iter_mut().map(|current_node| {
            let mut number_of_steps = 0;
            while !current_node.id.ends_with('Z') {
                let instruction = &self.instructions[number_of_steps % self.instructions.len()];
                let next_node_id = current_node.mappings.get(instruction).unwrap();
                *current_node = self.nodes.get(next_node_id).unwrap();

                number_of_steps += 1;
            }
            number_of_steps
        }).collect();

        // Functions stolen from euc_lib
        fn euc(mut d1: usize, mut d2: usize) -> usize {
            while d2 != 0 {
                (d1, d2) = (d2, d1 % d2);
            }
            return d1;
        }
        fn lcm(d1: usize, d2: usize) -> usize {
            &(d1 * d2) / euc(d1, d2)
        }
        fn lcm_from_vec(mut d: Vec<usize>) -> Result<usize, String> {
            if d.len() == 2 {
                return Ok(lcm(d[0], d[1]));
            } else if d.len() < 2 {
                return Err(
                    "critical error occured, length of vector smaller than 2, unable to calculate lcm"
                        .to_owned(),
                );
            }
            Ok(lcm(
                d.pop().expect("error occured calculating lcm"),
                match lcm_from_vec(d.clone()) {
                    Ok(res) => res,
                    Err(err) => return Err(err),
                },
            ))
        }
        lcm_from_vec(number_of_steps).unwrap() as usize
    }
}

pub fn run() -> usize {
    let map = Map::from(split_input_into_lines(INPUT));
    map.steps_until_end()
}
