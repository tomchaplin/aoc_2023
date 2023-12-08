use crate::common::io::AocRunError;
use crate::common::problem::Problem;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn get_problem(problem: u32) -> Result<Box<dyn Problem>, AocRunError> {
    match problem {
        01 => Ok(Box::new(day01::Solution {})),
        02 => Ok(Box::new(day02::Solution {})),
        03 => Ok(Box::new(day03::Solution {})),
        04 => Ok(Box::new(day04::Solution {})),
        _ => Err(AocRunError::UnregistedProblem(problem)),
    }
}