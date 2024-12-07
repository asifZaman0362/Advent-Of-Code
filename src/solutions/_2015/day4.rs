use crate::solutions::*;

pub struct Solver;

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let str = &input[0];
        let mut number = None;
        let mut number2 = None;
        for num in 0..usize::MAX {
            let digest = md5::compute(format!("{str}{num}").as_bytes());
            let serialised = format!("{:x}", digest);
            if serialised.starts_with("000000") && number2.is_none() {
                number2 = Some(num);
            }
            if serialised.starts_with("00000") && number.is_none() {
                number = Some(num);
            }
            if number2.is_some() && number.is_some() {
                break;
            }
        }
        (number.unwrap(), number2.unwrap())
    }
}
