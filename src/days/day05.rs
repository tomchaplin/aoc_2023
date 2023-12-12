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
    input: String,
    output: String,
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
        Self {
            input: raw_map.0 .0,
            output: raw_map.0 .1,
            sub_maps: raw_map.1.into_iter().map(SubMap::parse).collect(),
        }
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
        let locations = seeds.into_iter().map(|s| almanac.map(s));
        let min_loc = locations.min().unwrap();
        Some(min_loc.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        None
    }
}
