use std::{cmp::Reverse, collections::HashMap};

use crate::Problem;
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

fn char_value(c: char, j_is_joker: bool) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if j_is_joker {
                1
            } else {
                11
            }
        }
        'T' => 10,
        c => c.to_digit(10).unwrap(),
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    string: String,
}

fn build_counts(str: &String) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in str.chars() {
        if !counts.contains_key(&c) {
            counts.insert(c, 0);
        }
        let current_count = counts.get(&c).unwrap();
        counts.insert(c, current_count + 1);
    }
    counts
}

fn count_pair_to_hand_type(count_pair: (usize, usize)) -> HandType {
    match count_pair {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        (1, _) => HandType::HighCard,
        _ => panic!("Unrecognised hand"),
    }
}

impl Hand {
    fn hand_type(&self, j_is_joker: bool) -> HandType {
        let mut counts = build_counts(&self.string);
        let mut values = if j_is_joker {
            // Delete the jokers
            let n_jokers = *counts.get(&'J').unwrap_or(&0);
            counts.insert('J', 0);
            let mut values: Vec<_> = counts.values().copied().collect();
            values.sort_by_key(|v| Reverse(*v));
            // Add jokers to largest class
            values[0] += n_jokers;
            values
        } else {
            let mut values: Vec<_> = counts.values().copied().collect();
            values.sort_by_key(|v| Reverse(*v));
            values
        };
        values.push(0);
        count_pair_to_hand_type((values[0], values[1]))
    }

    fn hand_strength(&self, j_is_joker: bool) -> (u32, u32, u32, u32, u32, u32) {
        let chars: Vec<_> = self.string.chars().collect();
        (
            self.hand_type(j_is_joker).value(),
            char_value(chars[0], j_is_joker),
            char_value(chars[1], j_is_joker),
            char_value(chars[2], j_is_joker),
            char_value(chars[3], j_is_joker),
            char_value(chars[4], j_is_joker),
        )
    }
}

#[derive(Debug)]
struct Row {
    hand: Hand,
    bid: u32,
}

impl Row {
    fn parse(input: (String, u32)) -> Row {
        Row {
            hand: Hand { string: input.0 },
            bid: input.1,
        }
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    let p = parser!(lines(
        string(any_char+) " " u32
    ));
    let parsed_raw = p.parse(input).unwrap();
    parsed_raw.into_iter().map(Row::parse).collect()
}

fn solve(input: &str, j_is_joker: bool) -> u32 {
    let mut rows = parse_input(input);
    rows.sort_by_key(|row| row.hand.hand_strength(j_is_joker));
    rows.into_iter()
        .enumerate()
        .map(|(rank_minus_1, row)| (rank_minus_1 as u32 + 1) * row.bid)
        .sum()
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        Some(solve(input, false).to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        Some(solve(input, true).to_string())
    }
}
