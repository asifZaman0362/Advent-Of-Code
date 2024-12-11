use crate::solutions::*;

use r#macro::{solution, test_sol};

#[test]
fn sol() {
    test_sol!("125 17", 55312, 0);
}

solution!(2024, 11, usize, {
    for line in input.lines() {}
    let (part1, part2) = (0, 0);
    (part1, part2)
});
