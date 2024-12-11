use crate::solutions::{Input, Solution};

//pub struct Solver;

fn check(vec: &[i32]) -> bool {
    let mut prev = 0;
    let mut sign = 0;
    let mut safe = true;
    for (idx, section) in vec.iter().enumerate() {
        if idx == 0 {
            prev = *section;
            continue;
        }
        let diff = section - prev;
        let _sign = diff.signum();
        if idx == 1 {
            sign = _sign;
        }
        let val = diff.abs();
        if sign != _sign || !(1..=3).contains(&val) {
            safe = false;
            break;
        }
        prev = *section;
    }
    safe
}

/*impl Solution for Solver {
type Answer = usize;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 2, usize, {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines().map(|x| {
        x.split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    }) {
        if check(&line) {
            part1 += 1;
            part2 += 1;
            continue;
        }
        for i in 0..line.len() {
            let (x, y) = line.split_at(i);
            let mut vec = Vec::from(x);
            vec.extend_from_slice(&y[1..]);
            if check(&vec) {
                part2 += 1;
                break;
            }
        }
    }
    (part1, part2)
});
