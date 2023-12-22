use crate::Problem;
use aoc_parse::{parser, prelude::*};
use itertools::Itertools;
pub struct Solution {}

type Grid = Vec<Vec<char>>;
type Mask = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Grid {
    parser!(lines(any_char+)).parse(input).unwrap()
}

fn compute_bounds(grid: &Grid) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn get_deltas(c: &char) -> [Delta; 2] {
    let mut c = c.clone();
    if c == 'S' {
        // Hard-coded!
        c = '7'
    }
    match c {
        '|' => [Delta(-1, 0), Delta(1, 0)],
        '-' => [Delta(0, -1), Delta(0, 1)],
        'L' => [Delta(-1, 0), Delta(0, 1)],
        'J' => [Delta(-1, 0), Delta(0, -1)],
        '7' => [Delta(1, 0), Delta(0, -1)],
        'F' => [Delta(1, 0), Delta(0, 1)],
        _ => panic!("Bad pipe character"),
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Position(usize, usize);
#[derive(PartialEq, Clone, Copy)]
struct Delta(isize, isize);
#[derive(Debug)]
struct OutOfBoundsError;

impl Position {
    fn add(&self, delta: Delta, bounds: (usize, usize)) -> Result<Position, OutOfBoundsError> {
        let new = (self.0 as isize + delta.0, self.1 as isize + delta.1);
        if (new.0 < 0)
            || (new.0 >= bounds.0 as isize)
            || (new.1 < 0)
            || (new.1 >= bounds.1 as isize)
        {
            Err(OutOfBoundsError)
        } else {
            Ok(Position(new.0 as usize, new.1 as usize))
        }
    }
}

fn find_s(grid: &Grid) -> Position {
    let (rows, cols) = compute_bounds(grid);
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'S' {
                return Position(i, j);
            }
        }
    }
    panic!("Couldn't find S");
}

struct Walker {
    current: Position,
    previous: Position,
    bounds: (usize, usize),
}

impl Walker {
    fn init(pos: Position, bounds: (usize, usize)) -> Self {
        Self {
            current: pos,
            previous: pos,
            bounds,
        }
    }

    fn follow_pipe(&mut self, c: &char) {
        let deltas = get_deltas(c);
        // Should usually be exactly one position after filter unless self.current == self.previous
        let new_position = deltas
            .into_iter()
            .map(|d| self.current.add(d, self.bounds).unwrap())
            .filter(|new_pos| new_pos != &self.previous)
            .next()
            .unwrap();
        let current_store = self.current;
        self.current = new_position;
        self.previous = current_store;
    }
}

fn build_loop_mask(grid: &Grid) -> Mask {
    let bounds = compute_bounds(grid);
    let mut loop_mask = vec![vec![false; bounds.1]; bounds.0];
    let s_pos = find_s(&grid);
    let mut walker = Walker::init(s_pos, bounds);
    loop {
        let pipe = grid[walker.current.0][walker.current.1];
        walker.follow_pipe(&pipe);
        loop_mask[walker.current.0][walker.current.1] = true;
        if walker.current == s_pos {
            break;
        }
    }
    loop_mask
}

fn n_crosses_above(pos: &Position, grid: &Grid, loop_mask: &Mask) -> usize {
    let horizontal_connections = (0..(pos.0))
        .filter(|&i| loop_mask[i][pos.1])
        .flat_map(|i| get_deltas(&grid[i][pos.1]))
        .filter(|d| d.0 == 0);
    // Follow the path from pos to the top of the grid
    // How many times do you have to cross the loop?
    // Loop appears in chunks starting and ending at a horizontal connection
    // In each chunk, have to cross loop if and only if connection goes left -> right
    horizontal_connections
        .chunks(2)
        .into_iter()
        .filter_map(|mut ck| {
            let ck_0 = ck.next().unwrap();
            let ck_1 = ck.next().unwrap();
            if ck_0 == ck_1 {
                None
            } else {
                Some(())
            }
        })
        .count()
}

fn iter_grid_positions(bounds: (usize, usize)) -> impl Iterator<Item = Position> {
    (0..(bounds.0)).flat_map(move |i| (0..(bounds.1)).map(move |j| Position(i, j)))
}

impl Problem for Solution {
    #[allow(unused_variables)]
    fn solve_a(&self, input: &str) -> Option<String> {
        let grid = parse_input(input);
        let bounds = compute_bounds(&grid);
        let s_pos = find_s(&grid);
        let mut walker = Walker::init(s_pos, bounds);
        let mut n_steps = 0u64;
        loop {
            let pipe = grid[walker.current.0][walker.current.1];
            walker.follow_pipe(&pipe);
            n_steps += 1;
            if walker.current == s_pos {
                break;
            }
        }
        let furthest = n_steps.div_euclid(2);
        Some(furthest.to_string())
    }

    #[allow(unused_variables)]
    fn solve_b(&self, input: &str) -> Option<String> {
        let grid = parse_input(input);
        let bounds = compute_bounds(&grid);
        let loop_mask = build_loop_mask(&grid);

        let not_on_loop = |pos: &Position| !loop_mask[pos.0][pos.1];
        let odd_crosses_above =
            |pos: &Position| n_crosses_above(pos, &grid, &loop_mask).rem_euclid(2) == 1;

        let n_inside = iter_grid_positions(bounds)
            .filter(not_on_loop)
            .filter(odd_crosses_above)
            .count();

        Some(n_inside.to_string())
    }
}
