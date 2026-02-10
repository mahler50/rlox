use std::{cmp::Ordering, env};

use rlox::{
    error::RloxError,
    runner::{run_file, run_prompt},
};

fn main() -> Result<(), RloxError> {
    let args: Vec<String> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rlox [script]");
            // exit with wrong number of arguments.
            std::process::exit(64);
        }
        Ordering::Equal => run_file(&args[1]),
        Ordering::Less => run_prompt(),
    }
}
