use crate::solutions::*;

pub struct Solver;

struct Tuple(i64, i64);

impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Tuple) -> Self::Output {
        Tuple(rhs.0 + self.0, rhs.1 + self.1)
    }
}

impl std::iter::Sum for Tuple {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Tuple(0, 0), |prev, curr| prev + curr)
    }
}

impl Solution for Solver {
    type Answer = i64;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let sums = input
            .iter()
            .map(|line| line.split('x').map(i64_p).collect::<Vec<_>>())
            .map(|dimens| {
                if let &[l, b, h] = dimens.as_slice() {
                    let mut areas = [l * b, b * h, l * h];
                    let mut peris = [2 * (l + b), 2 * (b + h), 2 * (l + h)];
                    peris.sort();
                    areas.sort();
                    Tuple(
                        areas.iter().map(|s| 2 * s).sum::<i64>() + areas[0],
                        peris[0] + l * b * h,
                    )
                } else {
                    panic!()
                }
            })
            .sum::<Tuple>();
        (sums.0, sums.1)
    }
}
