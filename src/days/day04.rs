use std::collections::{HashMap, HashSet};

use crate::Problem;
use aoc_parse::{parser, prelude::*};

pub struct Solution {}

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    winning: Vec<usize>,
    played: Vec<usize>,
}

impl Card {
    fn n_wins(&self) -> usize {
        let winning_set: HashSet<_> = HashSet::from_iter(self.winning.iter().cloned());
        let played_set = HashSet::from_iter(self.played.iter().cloned());
        winning_set.intersection(&played_set).count()
    }

    fn score(&self) -> u32 {
        let n_wins = self.n_wins();
        if n_wins == 0 {
            return 0;
        }
        let base: u32 = 2;
        base.pow((n_wins - 1) as u32)
    }

    fn card_winnings(&self, max_idx: usize) -> impl Iterator<Item = usize> {
        let n_wins = self.n_wins();
        let start_wins = self.index + 1;
        let end_wins = start_wins + n_wins;
        (start_wins..end_wins).filter(move |&j| j < max_idx)
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    let p = parser!(
        lines(
            "Card" " "+ usize ":" " "+
            repeat_sep(usize, " "+)
            " "+ "|" " "+
            repeat_sep(usize, " "+)
        )
    );
    let parsed_lines = p.parse(input).unwrap();
    parsed_lines
        .into_iter()
        .map(|line| {
            let index = line.1 - 1;
            let winning = line.3;
            let played = line.6;
            Card {
                index,
                winning,
                played,
            }
        })
        .collect()
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let cards = parse_input(input);
        let total_score: u32 = cards.into_iter().map(|c| c.score()).sum();
        Some(total_score.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let cards = parse_input(input);
        let mut winnings_by_card: HashMap<usize, u32> = HashMap::new();
        let n_cards = cards.len();
        let mut total_winnings = n_cards as u32;
        for i in (0..n_cards).rev() {
            let mut i_winnings = 0;
            let card_winnings = cards[i].card_winnings(n_cards);
            for j in card_winnings {
                i_winnings += 1 + winnings_by_card.get(&j).unwrap();
            }
            winnings_by_card.insert(i, i_winnings);
            total_winnings += i_winnings;
        }
        Some(total_winnings.to_string())
    }
}
