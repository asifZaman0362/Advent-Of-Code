use crate::solutions::*;

//pub struct Solver;

// Behold... THE POWER TOWER!!!!!!!!!!!!!!!!
const POWERS: &[i64] = &[
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
];

const DIGITS: &[i64] = &[0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Num {
    len: i64,
    num: i64,
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl std::ops::Add<Num> for Num {
    type Output = Num;
    fn add(self, rhs: Num) -> Self::Output {
        let num = self.num + rhs.num;
        let len = self.len.max(rhs.len);
        Num {
            len: len + num / POWERS[len as usize],
            num,
        }
    }
}

impl std::ops::Mul<Num> for Num {
    type Output = Num;
    fn mul(self, rhs: Num) -> Self::Output {
        let num = self.num * rhs.num;
        if rhs.num == 0 || self.num == 0 {
            return Num { num: 0, len: 1 };
        }
        let len = (self.len + rhs.len) - 1;
        if num / POWERS[len as usize] > 9 {
            println!("{self:?}, {rhs:?}");
        }
        Num {
            len: len + DIGITS[(num / POWERS[len as usize]) as usize],
            num,
        }
    }
}

impl std::ops::BitOr<Num> for Num {
    type Output = Num;
    fn bitor(self, rhs: Num) -> Self::Output {
        let num = self.num * POWERS[rhs.len as usize] + rhs.num;
        let len = self.len + rhs.len;
        Num { num, len }
    }
}

impl From<&str> for Num {
    fn from(value: &str) -> Self {
        let num = i64_p(value);
        let len = value.len() as i64;
        Self { len, num }
    }
}

fn get_comb_2(items: &[Num]) -> HashSet<Num> {
    if items.len() == 2 {
        HashSet::from([
            (items[0] + items[1]),
            (items[0] * items[1]),
            (items[1] | items[0]),
        ])
    } else {
        let mut set = HashSet::with_capacity(3 ^ items.len());
        for item in get_comb_2(&items[1..]) {
            set.insert(items[0] + item);
            set.insert(items[0] * item);
            set.insert(item | items[0]);
        }
        set
    }
}

fn get_comb(items: &[Num]) -> HashSet<Num> {
    if items.len() == 2 {
        HashSet::from([(items[0] + items[1]), (items[0] * items[1])])
    } else {
        let mut set = HashSet::with_capacity(2 ^ items.len());
        for item in get_comb(&items[1..]) {
            set.insert(items[0] + item);
            set.insert(items[0] * item);
        }
        set
    }
}

fn check_solution(result: Num, ops: &[Num]) -> (bool, bool) {
    if get_comb(ops).contains(&result) {
        (true, false)
    } else {
        (false, get_comb_2(ops).contains(&result))
    }
}

fn solve0(input: Input) -> (i64, i64) {
    let mut sum1: Num = Num::from("0");
    let mut sum2: Num = Num::from("0");
    for line in input.lines() {
        if let Some((result, mut ops)) = line.split_once(":").map(|(res, ops)| {
            (
                Num::from(res),
                ops.trim()
                    .split(" ")
                    .map(Into::<Num>::into)
                    .collect::<Vec<_>>(),
            )
        }) {
            ops.reverse();
            let (part1, part2) = check_solution(result, &ops);
            if part1 {
                sum1 = sum1 + result;
            } else if part2 {
                sum2 = sum2 + result;
            }
        }
    }
    (sum1.num, sum1.num + sum2.num)
}

/*impl Solution for Solver {
type Answer = i64;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 7, i64, solve0(input));
