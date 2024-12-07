use crate::solutions::*;

pub struct Solver;

impl Solution for Solver {
    type Answer = i64;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let mut floor = 0;
        let mut basement = None;
        let mut idx = 0;
        for line in input {
            for char in line.bytes() {
                idx += 1;
                if char == b'(' {
                    floor += 1;
                } else {
                    floor -= 1;
                }
                if basement.is_none() && floor == -1 {
                    basement = Some(idx);
                }
            }
        }
        (floor, basement.unwrap())
    }
}