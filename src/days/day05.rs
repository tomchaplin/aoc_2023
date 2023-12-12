use rayon::prelude::*;
use std::ops::Range;

use crate::Problem;
use aoc_parse::{parser, prelude::*};

pub struct Solution {}

#[derive(Debug)]
struct SubMap {
    source_start: isize,
    dest_start: isize,
    len: isize,
}

impl SubMap {
    fn contains(&self, idx: isize) -> bool {
        (idx >= self.source_start) && (idx < self.source_start + self.len)
    }

    fn map(&self, input: isize) -> isize {
        input - (self.source_start - self.dest_start)
    }

    fn parse(raw: (isize, isize, isize)) -> Self {
        SubMap {
            dest_start: raw.0,
            source_start: raw.1,
            len: raw.2,
        }
    }
}

#[derive(Debug)]
struct ResourceMap {
    sub_maps: Vec<SubMap>,
}
impl ResourceMap {
    fn map(&self, input: isize) -> isize {
        for sub_map in self.sub_maps.iter() {
            if sub_map.contains(input) {
                return sub_map.map(input);
            }
        }
        return input;
    }

    fn parse(raw_map: ((String, String), Vec<(isize, isize, isize)>)) -> Self {
        let mut sub_maps: Vec<_> = raw_map.1.into_iter().map(SubMap::parse).collect();
        sub_maps.sort_by_key(|m| m.source_start);
        Self { sub_maps }
    }
}

#[derive(Debug)]
struct Almanac(Vec<ResourceMap>);

impl Almanac {
    fn map(&self, mut input: isize) -> isize {
        for r_map in self.0.iter() {
            input = r_map.map(input);
        }
        input
    }
}

struct Interval {
    start: isize,
    length: isize,
}

impl Interval {
    fn end_exclusive(&self) -> isize {
        self.start + self.length
    }

    fn interval_iter(&self) -> Range<isize> {
        println!("Starting interval");
        self.start..self.end_exclusive()
    }
}

fn parse_input(input: &str) -> (Vec<isize>, Almanac) {
    let seed_p = parser!(line("seeds: " repeat_sep(isize, " ")));
    let map_p = parser!(
        line(string(alpha+) "-to-" string(alpha+) " map:")
        lines(
            isize " " isize " " isize
        )
    );
    let input_p = parser!(
        section(seed_p)
        sections(map_p)
    );
    let parsed_raw = input_p.parse(input).unwrap();
    let seeds = parsed_raw.0;
    let almanac = Almanac(parsed_raw.1.into_iter().map(ResourceMap::parse).collect());
    (seeds, almanac)
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (seeds, almanac) = parse_input(input);
        let locations = seeds.into_iter().par_bridge().map(|s| almanac.map(s));
        let min_loc = locations.min().unwrap();
        Some(min_loc.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        // Brute-force attempt (incredibly stupid)
        let (seeds_and_lengths, almanac) = parse_input(input);
        let mut intervals: Vec<_> = seeds_and_lengths
            .chunks(2)
            .map(|pair| Interval {
                start: pair[0],
                length: pair[1],
            })
            .collect();
        intervals.sort_by_key(|it| it.start);
        for i in 0..(intervals.len() - 1) {
            assert!(intervals[i].end_exclusive() <= intervals[i + 1].start)
        }

        let possible_seeds = intervals.iter().flat_map(|it| it.interval_iter());

        let min_location = possible_seeds
            .par_bridge()
            .map(|s| almanac.map(s))
            .min()
            .unwrap();

        Some(min_location.to_string())
    }
}
