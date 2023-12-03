use std::io::{self, Write};

use crate::day_1::{solution_1, solution_2};

mod day_1;
mod day_2;
mod utils;

fn main() {
    match prompt("Select a challenge day: ") {
        1 => select_challenge(solution_1, solution_2),
        2 => select_challenge(day_2::solution1, || {}),
        _ => panic!("Out of bounds day selected!"),
    };
}

fn select_challenge(f1: impl Fn(), f2: impl Fn()) {
    match prompt("Select a challenge to run: ") {
        1 => f1(),
        2 => f2(),
        _ => panic!("Challenge out of bounds!"),
    };
}

fn prompt(message: &str) -> u8 {
    print!("{message}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim().parse::<u8>().unwrap()
}
