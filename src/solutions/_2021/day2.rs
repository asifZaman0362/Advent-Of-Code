use crate::solutions::*;

//pub struct Solver {}

//impl Solution for Solver {
//type Answer = i32;
//fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
r#macro::solution!(2021, 2, i32, {
    let mut pos = (0, 0);
    let mut aim = 0;
    for instruction in input.split('\n') {
        if let Some((dir, val)) = instruction.split_once(" ") {
            let val = val.parse::<i32>().unwrap();
            match dir {
                "forward" => {
                    pos.0 += val;
                    pos.1 += val * aim;
                }
                "up" => {
                    aim -= val;
                }
                "down" => {
                    aim += val;
                }
                _ => unreachable!(),
            }
        }
    }
    (pos.0 * pos.1, 0)
});
//}
