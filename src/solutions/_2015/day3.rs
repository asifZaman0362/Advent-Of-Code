use crate::solutions::*;

//pub struct Solver;

fn mov(pos: &(i64, i64), diff: &(i64, i64)) -> (i64, i64) {
    (pos.0 + diff.0, pos.1 + diff.1)
}

//impl Solution for Solver {
//type Answer = usize;
//fn solve(input: Input) -> (Self::Answer, Self::Answer) {
r#macro::solution!(2015, 3, usize, {
    let mut part1_pos = (0, 0);
    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);
    let mut p1_presents_delivered = HashSet::from([(0, 0)]);
    let mut presents_delivered = HashSet::from([(0, 0)]);
    let mut i = 0;
    for line in input.split('\n') {
        for dir in line.chars() {
            let mvmt = match dir {
                '>' => (1, 0),
                '^' => (0, 1),
                '<' => (-1, 0),
                'v' => (0, -1),
                _ => unreachable!(),
            };
            part1_pos = mov(&part1_pos, &mvmt);
            p1_presents_delivered.insert(part1_pos);
            if i == 0 {
                santa_pos = mov(&santa_pos, &mvmt);
                presents_delivered.insert(santa_pos);
            } else {
                robo_santa_pos = mov(&robo_santa_pos, &mvmt);
                presents_delivered.insert(robo_santa_pos);
            }
            i = (i + 1) % 2;
        }
    }
    (p1_presents_delivered.len(), presents_delivered.len())
});
//}
