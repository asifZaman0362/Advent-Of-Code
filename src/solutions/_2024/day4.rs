use crate::solutions::*;

//pub struct Solver;

// This code is NOT meant to be readable. I wanted to challenge myself by writing as much of
// the solution in a functional style as possible and hence the ugly code

fn find_directional(x: usize, y: usize, input: &[&[u8]], (dx, dy): (isize, isize)) -> bool {
    //let (mut x, mut y) = (x, y);
    "MAS"
        .bytes()
        .enumerate()
        .map_while(|(idx, c)| {
            input
                .get(next(y, dy, idx)?)
                .and_then(|line| line.get(next(x, dx, idx)?))
                .and_then(|&other| (c == other).then_some(()))
        })
        .count()
        == 3
}

fn find_xmas(x: usize, y: usize, input: &[&[u8]]) -> usize {
    (-1..=1)
        .flat_map(move |x| (-1..=1).map(move |y| (x, y)))
        .filter(|&(x, y)| x != 0 || y != 0)
        .filter(|&(i, j)| find_directional(x, y, input, (i, j)))
        .count()
}

fn find_x_mas(x: isize, y: isize, input: &[&[u8]]) -> bool {
    [1, -1].iter().all(|sign| {
        ["MS", "SM"].iter().any(|&str| {
            str.bytes()
                .zip([-1, 1].map(|dx| input[(y + sign * dx) as usize][(x + dx) as usize]))
                .all(|(l, r)| l == r)
        })
    })
}

fn solve1(input: &[&[u8]]) -> usize {
    input
        .iter()
        .skip(1)
        .take(input.len() - 2)
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .skip(1)
                .take(line.len() - 2)
                .enumerate()
                .filter(|&(x, char)| {
                    *char == b'A' && find_x_mas((x + 1) as isize, (y + 1) as isize, input)
                })
                .count()
        })
        .sum()
}

fn solve0(input: &[&[u8]]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &char)| (char == b'X'))
                .map(|(x, _)| find_xmas(x, y, input))
                .sum::<usize>()
        })
        .sum()
}

/*impl Solution for Solver {
type Answer = usize;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 4, usize, {
    let input = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    (solve0(&input), solve1(&input))
});
