use crate::Problem;
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

fn get_differences<'a>(input: &'a [i64]) -> impl Iterator<Item = i64> + 'a {
    input.windows(2).map(|w| w[1] - w[0])
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let p = parser!(lines(repeat_sep(i64, " ")));
    p.parse(input).unwrap()
}

fn all_zero(input: &[i64]) -> bool {
    input.iter().all(|&elem| elem == 0)
}

fn compute_extrapolated_end(input: Vec<i64>) -> i64 {
    let mut final_values: Vec<i64> = vec![];
    let mut vec_1 = input;
    let mut vec_2 = Vec::with_capacity(vec_1.len());
    let mut working_vec = &mut vec_1;
    let mut next_vec = &mut vec_2;
    loop {
        // Store the last values
        final_values.push(*working_vec.last().unwrap());
        // Clear the other vector
        next_vec.clear();
        // Fill the other vector with differences
        next_vec.extend(get_differences(&working_vec));
        // Break if done
        if all_zero(&next_vec) {
            break final_values.into_iter().sum();
        }
        // Swap the references
        let holding = next_vec;
        next_vec = working_vec;
        working_vec = holding;
    }
}

// As before but pull out first and different reduction
fn compute_extrapolated_begin(input: Vec<i64>) -> i64 {
    let mut final_values: Vec<i64> = vec![];
    let mut vec_1 = input;
    let mut vec_2 = Vec::with_capacity(vec_1.len());
    let mut working_vec = &mut vec_1;
    let mut next_vec = &mut vec_2;
    loop {
        final_values.push(*working_vec.first().unwrap());
        next_vec.clear();
        next_vec.extend(get_differences(&working_vec));
        if all_zero(&next_vec) {
            break final_values
                .into_iter()
                .rev()
                .reduce(|accum, elem| elem - accum)
                .unwrap();
        }
        let holding = next_vec;
        next_vec = working_vec;
        working_vec = holding;
    }
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let lines = parse_input(input);
        Some(
            lines
                .into_iter()
                .map(compute_extrapolated_end)
                .sum::<i64>()
                .to_string(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let lines = parse_input(input);
        Some(
            lines
                .into_iter()
                .map(compute_extrapolated_begin)
                .sum::<i64>()
                .to_string(),
        )
    }
}
