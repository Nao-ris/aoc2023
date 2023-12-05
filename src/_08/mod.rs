
use std::collections::HashSet;

use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Card {
    pub _id: u32,
    pub winning_number: HashSet<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn matching_numbers(&self) -> usize {
        self.numbers.iter().filter(|number| self.winning_number.contains(number)).count()
    }
}

fn line_to_card(line: String) -> Card {
    let regex_card = r"^Card[ ]+(?P<id>\d+): (?P<winning>[0-9 ]+) \| (?P<numbers>[0-9 ]+)$";
    let regex_game = Regex::new(regex_card).unwrap();

    if let Some(cap) = regex_game.captures(&line) {
        let id: u16 = cap.get(1).unwrap().as_str().parse().unwrap();
        let winning_numbers = cap.get(2).unwrap().as_str();
        let numbers = cap.get(3).unwrap().as_str();

        Card {
            _id: id as u32,
            winning_number: winning_numbers.split_whitespace().map(|n| n.parse().unwrap()).collect::<HashSet<u32>>(),
            numbers: numbers.split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<u32>>(),
        }
    } else {
        panic!("The following line could not be parsed: {}", line);
    }
}

pub fn run() -> usize {
    let cards: Vec<Card> = split_input_into_lines(INPUT)
        .into_iter()
        .map(| line: String | line_to_card(line))
        .collect();

    let mut number_of_cards: Vec<usize> = vec![1; cards.len()];
    for (index, card) in cards.into_iter().enumerate() {
        let number_of_current_card = number_of_cards[index];

        for i in 1..=card.matching_numbers() {
            let new_index = index + i;
            if new_index < number_of_cards.len() {
                number_of_cards[new_index] += number_of_current_card;
            }
        }
    }

    number_of_cards.into_iter().sum::<usize>()
}
