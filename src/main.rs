mod common;
mod days;

use common::{io::RunCode, problem::Problem};
use std::{env, str::FromStr};

pub fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() <= 2);

    if args.len() == 1 || args[1] == "all" {
        for i in 1.. {
            let res = RunCode::init_all(i as u32).run();
            match res {
                Err(common::io::AocRunError::UnregistedProblem(_)) => break,
                Ok(rc) => {
                    println!("Day {}", i);
                    rc.print()
                }
                Err(e) => println!("{}", e),
            }
        }
    } else {
        let res = RunCode::from_str(&args[1]).expect("Valid run code").run();
        match res {
            Ok(rc) => rc.print(),
            Err(e) => println!("{}", e),
        }
    }
}
