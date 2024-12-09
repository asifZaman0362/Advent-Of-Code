mod input;
use input::read_input;
use std::collections::HashMap;

use solutions::Solution;
type Solver = solutions::_2020::day1::Solver;

mod solutions;

const YEAR: u16 = 2020;
const DAY: u8 = 1;

fn main() -> anyhow::Result<()> {
    let begin = std::time::Instant::now();
    let input = read_input(false, YEAR, DAY)?;
    let (first, second) = Solver::solve(&input);
    let end = std::time::Instant::now();
    let time = end.duration_since(begin);
    dbg!(time);
    println!("{first}, {second}");
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    let begin = std::time::Instant::now();
    let input = read_input(true, YEAR, DAY)?;
    let sol = Solver::solve(&input);
    assert_eq!(sol, (514579, 241861950));
    let end = std::time::Instant::now();
    let time = end.duration_since(begin);
    dbg!(time);
    Ok(())
}
