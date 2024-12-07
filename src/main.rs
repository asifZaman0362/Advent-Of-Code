mod input;
use input::read_input;
use std::collections::HashMap;

use solutions::Solution;
type Solver = solutions::_2015::day2::Solver;

mod solutions;

const YEAR: u16 = 2015;
const DAY: u8 = 2;

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
    assert_eq!(sol, (58 + 43, 34 + 14));
    let end = std::time::Instant::now();
    let time = end.duration_since(begin);
    dbg!(time);
    Ok(())
}
