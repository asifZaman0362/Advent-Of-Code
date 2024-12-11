use crate::solutions::*;

//pub struct Solver;

//impl Solution for Solver {
//type Answer = u32;
//fn solve(input: Input) -> (Self::Answer, Self::Answer) {
r#macro::solution!(2020, 1, u32, {
    let set = input.split('\n').map(|x| u32_p(x)).collect::<HashSet<_>>();
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
});
//}
