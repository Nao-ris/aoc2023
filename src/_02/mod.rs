
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

fn match_and_replace(pattern: &str, replace: u8, input: &String, output: &mut Vec<Option<u8>>) {
    for (i, _) in input.match_indices(pattern) {
        output[i] = Some(replace);
    }
}

fn get_value(s: &String) -> usize {
    let mut digits = vec![None; s.len()];

    match_and_replace("zero", 0, s, &mut digits);
    match_and_replace("one", 1, s, &mut digits);
    match_and_replace("two", 2, s, &mut digits);
    match_and_replace("three", 3, s, &mut digits);
    match_and_replace("four", 4, s, &mut digits);
    match_and_replace("five", 5, s, &mut digits);
    match_and_replace("six", 6, s, &mut digits);
    match_and_replace("seven", 7, s, &mut digits);
    match_and_replace("eight", 8, s, &mut digits);
    match_and_replace("nine", 9, s, &mut digits);
    match_and_replace("0", 0, s, &mut digits);
    match_and_replace("1", 1, s, &mut digits);
    match_and_replace("2", 2, s, &mut digits);
    match_and_replace("3", 3, s, &mut digits);
    match_and_replace("4", 4, s, &mut digits);
    match_and_replace("5", 5, s, &mut digits);
    match_and_replace("6", 6, s, &mut digits);
    match_and_replace("7", 7, s, &mut digits);
    match_and_replace("8", 8, s, &mut digits);
    match_and_replace("9", 9, s, &mut digits);

    let digits: Vec<u8>  = digits.into_iter().filter_map(|d| d).collect();
    if digits.is_empty() {
        0
    } else {
        (digits.first().unwrap() * 10 + digits.last().unwrap()) as usize
    }
}

pub fn run() -> usize {
    split_input_into_lines(INPUT)
        .iter()
        .fold(0, |acc, s| {
            acc + get_value(s)
        })
}
