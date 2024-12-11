use crate::solutions::*;

//pub struct Solver;

fn get_scores(
    buf: &[&mut [u8]],
    vec: Vec2<usize>,
    c: u8,
    seen: &mut HashSet<(usize, usize)>,
) -> Vec2<usize> {
    let x_max = buf[0].len();
    let y_max = buf.len();
    let Vec2::<usize> { x, y } = vec;
    let mut score = Vec2::<usize>::from((0, 0));
    if buf[y][x] == c + 1 {
        if c + 1 == b'9' {
            if seen.insert((x, y)) {
                (1, 1).into()
            } else {
                (0, 1).into()
            }
        } else {
            if y > 0 {
                score += get_scores(buf, (x, y - 1).into(), c + 1, seen);
            }
            if y < y_max - 1 {
                score += get_scores(buf, (x, y + 1).into(), c + 1, seen);
            }
            if x > 0 {
                score += get_scores(buf, (x - 1, y).into(), c + 1, seen);
            }
            if x < x_max - 1 {
                score += get_scores(buf, (x + 1, y).into(), c + 1, seen);
            }
            score
        }
    } else {
        (0, 0).into()
    }
}

/*impl Solution for Solver {
type Answer = usize;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 10, usize, {
    let mut scores = Vec2::<usize>::from((0, 0));
    let buf: Vec<&mut [u8]> = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .map(|vec| vec.leak())
        .collect();
    for (y, line) in buf.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char == b'0' {
                let mut seen = HashSet::new();
                scores += get_scores(&buf, (x, y).into(), char - 1, &mut seen);
            }
        }
    }
    (scores.x, scores.y)
});
