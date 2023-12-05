
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

fn get_value(s: &String) -> usize {
    let mut first_number = None;
    let mut last_number = None;

    for c in s.chars() {
        if c.is_digit(10) {
            if first_number.is_none() {
                first_number = c.to_digit(10);
            }
            last_number = c.to_digit(10);
        }
    }

    let result_number = format!(
        "{}{}",
        first_number.unwrap_or_default(),
        last_number.unwrap_or_default()
    );
    result_number.parse().unwrap()
}

pub fn run() -> usize {
    split_input_into_lines(INPUT)
        .iter()
        .fold(0, |acc, s| {
            acc + get_value(s)
        })
}
