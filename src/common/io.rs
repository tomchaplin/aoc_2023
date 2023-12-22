use aoc_parse::{parser, prelude::*};
use colored::Colorize;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use crate::days;

#[derive(Debug)]
pub struct RunCode {
    problem: u32,
    run_a: bool,
    run_b: bool,
    solution_a: Option<String>,
    solution_b: Option<String>,
    as_example: bool,
}

impl RunCode {
    pub fn init_all(problem: u32) -> Self {
        Self {
            problem,
            run_a: true,
            run_b: true,
            solution_a: None,
            solution_b: None,
            as_example: false,
        }
    }
}

impl FromStr for RunCode {
    type Err = aoc_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ab_parser = parser!({
            "a" => (true, false),
             "b" => (false, true),
             "ab" => (true, true),
             "" => (true, true)
        });
        let example_parser = parser!({"~" => true, "" => false});
        let code_parser = parser!(example_parser u32 ab_parser);
        let (as_example, problem, (run_a, run_b)) = code_parser.parse(s)?;
        Ok(RunCode {
            problem,
            run_a,
            run_b,
            solution_a: None,
            solution_b: None,
            as_example,
        })
    }
}

fn print_solution(prefix: &str, solution: &Option<String>) {
    let solution_str = match solution {
        Some(s) => s.to_string().bold().cyan(),
        None => "TODO".to_string().dimmed(),
    };
    println!("{prefix} : {solution_str}")
}

pub enum AocRunError {
    NoFile(String),
    UnregistedProblem(u32),
}

impl Display for AocRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let explanation = match self {
            AocRunError::NoFile(path) => format!("Could not find problem input at path {path}"),
            AocRunError::UnregistedProblem(problem) => {
                format!("No struct registered for problem {problem}")
            }
        };
        explanation.bold().red().fmt(f)
    }
}

impl RunCode {
    fn get_input(&self) -> Result<String, AocRunError> {
        let foldername = if self.as_example {
            "examples"
        } else {
            "inputs"
        };
        let problem = self.problem;
        let path = format!("./data/{foldername}/{problem:02}.txt");
        fs::read_to_string(&path).map_err(|_e| AocRunError::NoFile(path))
    }

    pub fn run(mut self) -> Result<Self, AocRunError> {
        let problem = days::get_problem(self.problem)?;
        let input = self.get_input()?;
        if self.run_a {
            self.solution_a = problem.solve_a(&input);
        }
        if self.run_b {
            self.solution_b = problem.solve_b(&input);
        }
        Ok(self)
    }

    pub fn print(&self) {
        if self.run_a {
            print_solution("A", &self.solution_a);
        }
        if self.run_b {
            print_solution("B", &self.solution_b);
        }
    }
}
