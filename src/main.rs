mod input;
use input::read_input;
use std::collections::HashMap;

use solutions::Solution;
type Solver = solutions::_2022::day7::Solver;

mod solutions;

const YEAR: u16 = 2022;
const DAY: u8 = 7;

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
    assert_eq!(sol, (95437, 24933642));
    Ok(())
}
