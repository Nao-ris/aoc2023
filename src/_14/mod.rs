
use std::collections::HashMap;

use regex::Regex;

use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct InputHand {
    cards: Vec<Card>,
    bid: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Card {
    J = 0,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct TypedHand {
    hand: InputHand,
    hand_type: HandType,
}

impl From<char> for Card {
    fn from(char: char) -> Self {
        match char {
            '2' => Card::Number2,
            '3' => Card::Number3,
            '4' => Card::Number4,
            '5' => Card::Number5,
            '6' => Card::Number6,
            '7' => Card::Number7,
            '8' => Card::Number8,
            '9' => Card::Number9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Unknown card"),
        }
    }
}

impl From<String> for InputHand {
    fn from(line: String) -> Self {
        let regex_hand = r"^([2-9TJQKA]{5}) ([0-9]+)$";
        let regex_hand = Regex::new(regex_hand).unwrap();
        let hand_str = regex_hand.captures(&line).unwrap().get(1).unwrap().as_str();
        let bid_str = regex_hand.captures(&line).unwrap().get(2).unwrap().as_str();

        InputHand {
            cards: hand_str.chars().map(Card::from).collect(),
            bid: bid_str.parse().unwrap(),
        }
    }
}

impl From<InputHand> for TypedHand {
    fn from(hand: InputHand) -> Self {
        let mut cards_map: HashMap<Card, u8> = HashMap::new();
        let mut number_of_j = 0;
        for card in hand.cards.iter() {
            if *card == Card::J {
                number_of_j += 1;
            } else {
                if let Some(count) = cards_map.get_mut(card) {
                    *count += 1;
                } else {
                    cards_map.insert(card.clone(), 1);
                }
            }
        }

        let mut repartition = cards_map.into_values().collect::<Vec<u8>>();
        repartition.sort();
        let hand_type = match repartition.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                match number_of_j {
                    2 | 1 => HandType::ThreeOfAKind,
                    _ => {
                        if *repartition.last().unwrap() == 3 {
                            HandType::ThreeOfAKind
                        } else {
                            HandType::TwoPair
                        }
                    },
                }
            },
            2 => {
                match number_of_j {
                    3 | 2 => HandType::FourOfAKind,
                    1 => {
                        if *repartition.last().unwrap() == 3 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    },
                    _ => {
                        if *repartition.last().unwrap() == 4 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    }
                }
            },
            1 | 0 => HandType::FiveOfAKind,
            _value => panic!("Impossible: {}", _value),
        };

        TypedHand {
            hand,
            hand_type,
        }
    }
}

pub fn run() -> usize {
    let mut hands = split_input_into_lines(INPUT)
        .into_iter()
        .map(InputHand::from)
        .map(TypedHand::from)
        .collect::<Vec<TypedHand>>();

    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            a.hand.cards.partial_cmp(&b.hand.cards).unwrap()
        } else {
            a.hand_type.partial_cmp(&b.hand_type).unwrap()
        }
    });

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank + 1) * hand.hand.bid)
}
