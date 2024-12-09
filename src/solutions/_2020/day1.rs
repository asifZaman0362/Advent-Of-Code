use crate::solutions::*;

pub struct Solver;

impl Solution for Solver {
    type Answer = u32;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let set = input
            .iter()
            .map(|x| u32_p(x.as_str()))
            .collect::<HashSet<_>>();
        (
            set.iter()
                .find_map(|&x| set.get(&(2020 - x)).map(|&y| x * y))
                .unwrap(),
            set.iter()
                .find_map(|&x| {
                    let r = 2020 - x;
                    set.iter()
                        .filter(|&&x| x < r)
                        .find_map(|&y| set.get(&r.abs_diff(y)).map(|&z| x * y * z))
                })
                .unwrap(),
        )
    }
}
