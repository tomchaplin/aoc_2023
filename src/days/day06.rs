use std::{iter::zip, ops::RangeInclusive};

use crate::Problem;
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    #[allow(dead_code)]
    fn iter_options(&self) -> RangeInclusive<u64> {
        0..=self.time
    }

    #[allow(dead_code)]
    fn score_option(&self, hold_time: u64) -> u64 {
        let speed = hold_time;
        let time_remaining = self.time - hold_time;
        speed * time_remaining
    }

    #[allow(dead_code)]
    fn n_ways_to_beat(&self) -> usize {
        self.iter_options()
            .map(|option| self.score_option(option))
            .filter(|&score| score > self.distance)
            .count()
    }

    fn solve_quadratic(&self) -> (f64, f64) {
        let disc = self.time * self.time - 4 * self.distance;
        let sqrt_disc = (disc as f64).sqrt();
        let minus_b = self.time as f64;
        let two_a = 2f64;
        let sol_1 = (minus_b - sqrt_disc) / two_a;
        let sol_2 = (minus_b + sqrt_disc) / two_a;
        (sol_1, sol_2)
    }

    fn n_ways_to_beat_v2(&self) -> u64 {
        let (sol_1, sol_2) = self.solve_quadratic();
        let first_beat = sol_1.ceil() as u64;
        let last_beat = sol_2.floor() as u64;
        last_beat - first_beat + 1
    }
}

fn parse_input_a(input: &str) -> Vec<Record> {
    let p = parser!(
        line("Time:" " "+ repeat_sep(u64," "+))
        line("Distance:" " "+ repeat_sep(u64," "+))
    );
    let parsed = p.parse(input).unwrap();
    let times = parsed.0 .1;
    let distances = parsed.1 .1;
    zip(times.into_iter(), distances.into_iter())
        .map(|(time, distance)| Record { time, distance })
        .collect()
}

fn ignore_kerning(input: String) -> u64 {
    let mut time = input;
    time.retain(|c| !c.is_whitespace());
    time.parse().unwrap()
}

fn parse_input_b(input: &str) -> Record {
    let p = parser!(
        line("Time:" string(any_char+))
        line("Distance:" string(any_char+))
            );
    let parsed = p.parse(input).unwrap();
    Record {
        time: ignore_kerning(parsed.0),
        distance: ignore_kerning(parsed.1),
    }
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let records = parse_input_a(input);
        let answer: u64 = records.into_iter().map(|r| r.n_ways_to_beat_v2()).product();
        Some(answer.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let record = parse_input_b(input);
        Some(record.n_ways_to_beat_v2().to_string())
    }
}
