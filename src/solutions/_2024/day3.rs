use crate::solutions::{i64_p, Input, Solution};
use regex;

pub struct Solver;

// aka part1 lol
fn calculate(input: &str) -> i64 {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |prev, (_, [a, b])| prev + i64_p(a) * i64_p(b))
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut remaining = input;
    while let Some((enabled, rest)) = remaining.split_once("don't()") {
        sum += calculate(enabled);
        remaining = rest.split_once("do()").map_or("", |(_, next)| next);
    }
    sum + calculate(remaining)
}

impl Solution for Solver {
    type Answer = i64;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let input = input.join(" ");
        (calculate(&input), part2(&input))
    }
}
