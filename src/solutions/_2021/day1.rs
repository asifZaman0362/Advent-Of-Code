use crate::solutions::*;

//pub struct Solver {}

//impl Solution for Solver {
//type Answer = u32;
r#macro::solution!(2021, 1, u32, {
    //fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
    let mut prev: Option<u32> = None;
    let mut count = 0;
    let input = input.split('\n').collect::<Vec<_>>();
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
});
//}
