use crate::Solution;

pub struct Solver {}

impl Solution for Solver {
    type Answer = u32;
    fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
        let mut prev: Option<u32> = None;
        let mut count = 0;
        for window in input.windows(3) {
            let sum = window.iter().map(|x| x.parse::<u32>().unwrap()).sum();
            if let Some(prev) = prev {
                if prev < sum {
                    count += 1;
                }
            }
            prev = Some(sum);
        }
        (count, 0)
    }
}
