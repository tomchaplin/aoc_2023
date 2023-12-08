#![feature(type_alias_impl_trait)]

mod common;
mod days;

use common::{io::RunCode, problem::Problem};
use std::{env, str::FromStr};

pub fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2);
    let run_code = RunCode::from_str(&args[1]).expect("Valid run code");
    let res = run_code.run();
    if let Err(e) = res {
        println!("{}", e)
    }
}
