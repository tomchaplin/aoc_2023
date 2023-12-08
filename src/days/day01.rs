use crate::Problem;
use regex::Regex;

pub struct Solution {}

fn parse_digit_string(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        c => c.parse().expect("Should be single digit"),
    }
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let output_lines = input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
            .map(|mut digits| {
                let first = digits.next().unwrap();
                let last = digits.last().unwrap_or(first);
                first * 10 + last
            });
        Some(output_lines.sum::<u32>().to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let lines = input.lines();
        let re = Regex::new("^(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
        let line_values = lines.map(|line| {
            let mut matches = (0..line.len()).filter_map(|i| {
                let substr = &line[i..];
                re.find(substr)
            });
            let first = matches.next().unwrap();
            let last = matches.last().unwrap_or(first);
            let tens = parse_digit_string(first.as_str());
            let units = parse_digit_string(last.as_str());
            tens * 10 + units
        });
        Some(line_values.sum::<u32>().to_string())
    }
}
