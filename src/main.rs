mod input;
use input::read_input;
use std::collections::HashMap;

mod solutions;

use solutions::Solution;
type Solver = solutions::_2024::day11::Solver;

fn main() -> anyhow::Result<()> {
    let begin = std::time::Instant::now();
    let (year, day) = Solver::date();
    let input = read_input(false, year, day)?;
    let (first, second) = Solver::solve(&input);
    let end = std::time::Instant::now();
    let time = end.duration_since(begin);
    dbg!(time);
    println!("{first}, {second}");
    Ok(())
}
