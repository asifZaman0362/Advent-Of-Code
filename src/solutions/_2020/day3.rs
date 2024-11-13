use crate::solutions::{Input, Solution};
use crate::HashMap;

pub struct Solver {}

impl Solution for Solver {
    type Answer = u64;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let mut pos = [0, 0, 0, 0, 0];
        let mut incs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut trees: HashMap<(usize, usize), u64> = HashMap::new();
        for (idx, line) in input.iter().enumerate().skip(1) {
            for (inc_idx, (x, y)) in incs.iter_mut().enumerate() {
                if idx % *y == 0 {
                    pos[inc_idx] += *x;
                    if line.as_bytes()[pos[inc_idx] % line.len()] == b'#' {
                        if let Some(val) = trees.get_mut(&(*x, *y)) {
                            *val += 1;
                        } else {
                            trees.insert((*x, *y), 1);
                        }
                    }
                }
            }
        }
        (*trees.get(&(3, 1)).unwrap(), trees.values().product())
    }
}
