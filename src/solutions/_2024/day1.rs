use crate::solutions::*;

pub struct Solver;

fn solve0(input: &crate::solutions::Input) -> i32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .iter()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (i32_p(a), i32_p(b)))
                .unwrap()
        })
        .unzip();
    left.sorted()
        .zip(right.sorted())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn solve1(input: crate::solutions::Input) -> i32 {
    let mut cts = [0u16; 100_000];
    input
        .iter()
        .map(|l| {
            l.split_once("   ")
                .map(|(x, y)| {
                    cts[usize_p(y)] += 1;
                    i32_p(x)
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|&x| cts[x as usize] as i32 * x)
        .sum()
}

impl Solution for Solver {
    type Answer = i32;
    fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
        (solve0(&input), solve1(input))
    }
}
