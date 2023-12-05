
use std::{cell::RefCell, rc::Rc};

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

type Number = Rc<RefCell<usize>>;
#[derive(Clone, Debug)]
enum PointType {
    None,
    Symbol,
    Number(Number),
}
type Line = Vec<PointType>;
type Schematic = Vec<Line>;

fn chars_to_schematics(chars: Vec<char>) -> Line {
    let mut previous_char = None;

    chars.into_iter().map(| char | {
        match char {
            '.' => {
                previous_char = None;
                PointType::None
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let value = char.to_digit(10).unwrap();

                let new_point = if let Some(PointType::Number(n)) = previous_char.as_mut() {
                    let new_value = 10 * *n.borrow() + (value as usize);
                    n.replace(new_value);

                    PointType::Number(n.clone())
                } else {
                    PointType::Number(Rc::new(RefCell::new(value as usize)))
                };

                previous_char = Some(new_point.clone());
                new_point
            }
            _ => {
                previous_char = None;
                PointType::Symbol
            },
        }
    }).collect()
}

fn there_is_a_symbol_around(schematic: &Schematic, x: usize, y: usize) -> bool {
    let x_c = x as i64;
    let y_c = y as i64;

    let points_to_check = vec![
        (x_c - 1, y_c - 1),
        (x_c    , y_c - 1),
        (x_c + 1, y_c - 1),
        (x_c - 1, y_c    ),
        (x_c + 1, y_c    ),
        (x_c - 1, y_c + 1),
        (x_c    , y_c + 1),
        (x_c + 1, y_c + 1),
    ];
    points_to_check.into_iter().any(|(x, y)| {
        if x >= 0 && (x as usize) < schematic[0].len() && y >= 0 && (y as usize) < schematic.len() {
            matches!(&schematic[y as usize][x as usize], PointType::Symbol)
        } else {
            false
        }
    })
}

fn add_to_selected_values(rc_set: &mut Vec<Number>, new_value: &Number) {
    let already_in_set = rc_set.iter().find(|n| Rc::ptr_eq(n, &new_value)).is_some();
    if !already_in_set {
        rc_set.push(new_value.clone());
    }
}

fn sum_part_numbers(mut schematic: Schematic) -> usize {
    let mut rc_set: Vec<Number> = vec![];

    let line_size = schematic[0].len();
    for y in 0..schematic.len() {
        for x in 0..line_size {
            if matches!(&schematic[y][x], PointType::Number(_)) {
                if there_is_a_symbol_around(&schematic, x , y) {
                    if let PointType::Number(value) = &mut schematic[y as usize][x as usize] {
                        add_to_selected_values(&mut rc_set, &value);
                    }
                }

            }
        }
    }

    rc_set.into_iter().fold(0, |acc, value| acc + *value.borrow())
}

pub fn run() -> usize {
    let schematic: Schematic = split_input_into_lines(INPUT)
        .into_iter()
        .map(| line | line.chars().collect::<Vec<char>>())
        .map(| chars | chars_to_schematics(chars))
        .collect();

    sum_part_numbers(schematic)
}
