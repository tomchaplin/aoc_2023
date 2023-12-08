use crate::Problem;
use aoc_parse::{parser, prelude::*};

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn empty() -> Self {
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn add(&mut self, count: u32, color: &str) {
        match color {
            "red" => self.red += count,
            "green" => self.green += count,
            "blue" => self.blue += count,
            _ => panic!("Unrecognised color"),
        }
    }

    fn all_leq_than(&self, other: &Self) -> bool {
        (self.red <= other.red) && (self.green <= other.green) && (self.blue <= other.blue)
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    index: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn min_ref_hand(&self) -> Hand {
        let red = self.hands.iter().map(|h| h.red).max().unwrap_or(0);
        let blue = self.hands.iter().map(|h| h.blue).max().unwrap_or(0);
        let green = self.hands.iter().map(|h| h.green).max().unwrap_or(0);
        Hand { red, blue, green }
    }
}

fn build_game(input: (u32, Vec<Vec<(u32, String)>>)) -> Game {
    let index = input.0;
    let hands_raw = input.1;
    let hands = hands_raw
        .into_iter()
        .map(|hand| {
            let mut new_hand = Hand::empty();
            for (count, color) in hand {
                new_hand.add(count, color.as_str())
            }
            new_hand
        })
        .collect();
    Game { index, hands }
}

fn parse_games(input: &str) -> Vec<Game> {
    let hand_p = parser!(
        repeat_sep(u32 " " string(alpha+) , ", ")
    );
    let p = parser!(
        lines(
            "Game " u32 ": " repeat_sep(hand_p, "; ")
        )
    );
    p.parse(input)
        .unwrap()
        .into_iter()
        .map(build_game)
        .collect()
}

pub struct Solution {}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let games = parse_games(input);
        let ref_hand = Hand {
            red: 12,
            green: 13,
            blue: 14,
        };
        let possible_games = games
            .into_iter()
            .filter(|game| game.hands.iter().all(|h| h.all_leq_than(&ref_hand)));
        Some(possible_games.map(|g| g.index).sum::<u32>().to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let games = parse_games(input);
        let total_power = games
            .into_iter()
            .map(|g| g.min_ref_hand().power())
            .sum::<u32>();
        Some(total_power.to_string())
    }
}
