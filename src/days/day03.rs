use crate::Problem;
use std::{collections::HashSet, iter::zip};
pub struct Solution {}

#[derive(PartialEq)]
enum Spot {
    Symbol(char),
    Space,
    Digit(char),
}

impl Spot {
    fn from_char(c: char) -> Self {
        if c == '.' {
            return Spot::Space;
        }
        if c.is_digit(10) {
            return Spot::Digit(c);
        }
        return Spot::Symbol(c);
    }
}

type Grid<T> = Vec<Vec<T>>;

fn parse_grid(input: &str) -> Grid<Spot> {
    input
        .lines()
        .map(|l| l.chars().map(Spot::from_char).collect())
        .collect()
}

fn build_init_mask(input: &Grid<Spot>) -> Grid<bool> {
    input
        .iter()
        .map(|line| line.iter().map(|s| matches!(s, Spot::Symbol(_))).collect())
        .collect()
}

fn get_neighbours(
    coords: (usize, usize),
    bounds: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    let i = coords.0 as isize;
    let j = coords.1 as isize;
    let n_rows = bounds.0 as isize;
    let n_cols = bounds.1 as isize;
    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .into_iter()
    .filter(move |&(a, b)| (0 <= a) && (a < n_rows) && (0 <= b) && (b < n_cols))
    .map(|(a, b)| (a as usize, b as usize))
}

fn get_bounds(grid: &Grid<Spot>) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn propogate_mask(mask: &mut Grid<bool>, grid: &Grid<Spot>) -> u32 {
    let mut changes = 0;
    let (n_rows, n_cols) = get_bounds(grid);
    // Find currently true-masked squares
    for i in 0..n_rows {
        for j in 0..n_cols {
            if !mask[i][j] {
                continue;
            }
            for (ni, nj) in get_neighbours((i, j), (n_rows, n_cols)) {
                // Propogate mask to non-space spots and count changes
                if mask[ni][nj] {
                    continue;
                }
                if grid[ni][nj] == Spot::Space {
                    continue;
                }
                mask[ni][nj] = true;
                changes += 1;
            }
        }
    }
    changes
}

fn find_gears<'a>(grid: &'a Grid<Spot>) -> impl Iterator<Item = (usize, usize)> + 'a {
    let (n_rows, n_cols) = get_bounds(&grid);

    (0..n_rows)
        .flat_map(move |i| (0..n_cols).map(move |j| (i, j)))
        .filter(|&(i, j)| matches!(grid[i][j], Spot::Symbol('*')))
}

fn get_number_at(grid: &Grid<Spot>, pos: (usize, usize)) -> (u32, (usize, usize)) {
    let (i, j) = pos;
    let (_n_rows, n_cols) = get_bounds(&grid);
    let mut jmin = j as isize;
    let mut jmax = j as isize;
    while (jmin >= 0) && (matches!(grid[i][jmin as usize], Spot::Digit(_))) {
        jmin -= 1;
    }
    jmin += 1;
    let jmin = jmin as usize;
    while (jmax < n_cols as isize) && (matches!(grid[i][jmax as usize], Spot::Digit(_))) {
        jmax += 1;
    }
    jmax -= 1;
    let jmax = jmax as usize;
    let digits: String = grid[i][jmin..=jmax]
        .iter()
        .map(|spot| match spot {
            Spot::Digit(c) => *c,
            _ => panic!(),
        })
        .collect();
    let value = digits.parse::<u32>().unwrap();
    (value, (i, jmin))
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let grid = parse_grid(input);
        let mut mask = build_init_mask(&grid);
        loop {
            let changes = propogate_mask(&mut mask, &grid);
            if changes == 0 {
                break;
            }
        }
        let masked_lines: Vec<String> = zip(grid, mask)
            .map(|(grid_line, mask_line)| {
                zip(grid_line.iter(), mask_line.iter())
                    .map(|(spot, mask)| {
                        if *mask {
                            match spot {
                                Spot::Symbol(_) => ' ',
                                Spot::Space => ' ',
                                Spot::Digit(c) => *c,
                            }
                        } else {
                            ' '
                        }
                    })
                    .collect()
            })
            .collect();

        let sum_of_ids = masked_lines
            .iter()
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|num_str| num_str.parse::<u32>().unwrap())
            })
            .sum::<u32>();

        Some(sum_of_ids.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let grid = parse_grid(input);
        let (n_rows, n_cols) = get_bounds(&grid);
        let mut accum = 0;
        for (gi, gj) in find_gears(&grid) {
            let mut ratio = 1;
            let mut adjacent_numbers = HashSet::new();
            for (ni, nj) in get_neighbours((gi, gj), (n_rows, n_cols)) {
                if !matches!(grid[ni][nj], Spot::Digit(_)) {
                    continue;
                }
                let (value, (start_i, start_j)) = get_number_at(&grid, (ni, nj));
                if adjacent_numbers.contains(&(start_i, start_j)) {
                    continue;
                }
                ratio *= value;
                adjacent_numbers.insert((start_i, start_j));
            }
            if adjacent_numbers.len() == 2 {
                accum += ratio;
            }
        }
        Some(accum.to_string())
    }
}
