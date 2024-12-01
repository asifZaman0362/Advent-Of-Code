use crate::solutions::Solution;

pub struct Solver;

fn solve0(input: &crate::solutions::Input) -> i32 {
    let mut list_left = vec![];
    let mut list_right = vec![];
    for line in input.iter() {
        if let Some((first, second)) = line.split_once(" ") {
            list_left.push(first.trim().parse::<i32>().unwrap());
            list_right.push(second.trim().parse::<i32>().unwrap());
        }
    }
    list_left.sort();
    list_right.sort();
    list_left
        .iter()
        .zip(list_right.iter())
        .map(|(&x, &y)| (x - y).abs())
        .sum::<i32>()
}

fn solve1(input: crate::solutions::Input) -> i32 {
    let mut numbers = vec![];
    let mut counts = std::collections::HashMap::<i32, i32>::new();
    for line in input {
        if let Some((first, second)) = line.split_once(" ") {
            numbers.push(first.trim().parse::<i32>().unwrap());
            let second = second.trim().parse::<i32>().unwrap();
            if let Some(count) = counts.get_mut(&second) {
                *count += 1;
            } else {
                counts.insert(second, 1);
            }
        }
    }
    numbers
        .iter()
        .map(|x| counts.get(x).map_or(0, |&y| *x * y))
        .sum()
}

impl Solution for Solver {
    type Answer = i32;
    fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
        (solve0(&input), solve1(input))
    }
}
