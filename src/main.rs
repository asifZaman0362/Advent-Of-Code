mod input;
use input::read_input;
use std::collections::HashMap;

use solutions::Solution;
type Solver = solutions::_2024::day1::Solver;

mod solutions;

const YEAR: u16 = 2024;
const DAY: u8 = 1;

fn main() -> anyhow::Result<()> {
    let input = read_input(false, YEAR, DAY)?;
    let (first, second) = Solver::solve(&input);
    println!("{first}, {second}");
    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    let input = read_input(true, YEAR, DAY)?;
    let sol = Solver::solve(&input);
    assert_eq!(sol, (11, 31));
    Ok(())
}
