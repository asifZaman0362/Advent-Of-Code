use crate::solutions::*;
use std::collections::VecDeque;

//pub struct Solver;

#[derive(Clone)]
enum Token {
    Old,
    Value(u128),
    Operation(Operation),
}

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u128>,
    tokens: Vec<Token>,
    divisibility: u128,
    decision: [u128; 2],
    inspection_count: u128,
}

fn evaluate(token: &Token, stress: u128) -> u128 {
    match token {
        Token::Old => stress,
        Token::Value(val) => *val,
        _ => unreachable!("bad code!"),
    }
}

impl Monkey {
    fn new(block: &[&str]) -> Self {
        let items = block[0]
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(", ")
            .map(|x| x.parse::<u128>().unwrap())
            .collect();
        let tokens = block[1]
            .split_once(':')
            .unwrap()
            .1
            .split_once(" = ")
            .unwrap()
            .1
            .split(' ')
            .map(|x| match x {
                "old" => Token::Old,
                "*" => Token::Operation(Operation::Multiply),
                "+" => Token::Operation(Operation::Add),
                x => Token::Value(x.parse::<u128>().unwrap()),
            })
            .collect();
        let divisibility = block[2]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let true_branch = block[3]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let false_branch = block[4]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<u128>()
            .unwrap();
        let decision = [true_branch, false_branch];
        Self {
            items,
            tokens,
            divisibility,
            decision,
            inspection_count: 0,
        }
    }

    fn inspect(&mut self, _mod: u128, div: u128) -> Option<(u128, u128)> {
        self.items.pop_front().map(|x| {
            self.inspection_count += 1;
            let (stress, next) = self.test(x, _mod, div);
            (stress, self.decision[next as usize])
        })
    }

    fn test(&self, stress: u128, _mod: u128, div: u128) -> (u128, u128) {
        let stress = (match self.tokens[1] {
            Token::Operation(Operation::Multiply) => stress * evaluate(&self.tokens[2], stress),
            Token::Operation(Operation::Add) => stress + evaluate(&self.tokens[2], stress),
            _ => unreachable!("bad code!"),
        } / div)
            % _mod;
        let next = match stress % self.divisibility {
            0 => 0,
            _ => 1,
        };
        (stress, next)
    }
}

fn simulate_round(monkeys: &mut [Monkey], _mod: u128, div: u128) {
    for i in 0..monkeys.len() {
        let mut transfers = vec![];
        while let Some((item, next)) = monkeys[i].inspect(_mod, div) {
            transfers.push((item, next));
        }
        for (item, next) in transfers {
            monkeys[next as usize].items.push_back(item);
        }
    }
}

fn solve0(input: &Vec<&str>) -> (u128, u128) {
    let mut monkeys = vec![];
    let mut divisors = std::collections::HashSet::new();
    for block in input.windows(6).step_by(6) {
        let monkey = Monkey::new(&block[1..]);
        divisors.insert(monkey.divisibility);
        monkeys.push(monkey)
    }
    let lcm = divisors.iter().product();
    let mut clones = monkeys.clone();
    for _ in 0..20 {
        simulate_round(&mut clones, lcm, 3);
    }
    clones.sort_by(|x, y| y.inspection_count.cmp(&x.inspection_count));
    let first = clones[0].inspection_count * clones[1].inspection_count;
    for _ in 0..10000 {
        simulate_round(&mut monkeys, lcm, 1);
    }
    monkeys.sort_by(|x, y| y.inspection_count.cmp(&x.inspection_count));
    (
        first,
        monkeys[0].inspection_count * monkeys[1].inspection_count,
    )
}

/*impl Solution for Solver {
    type Answer = u128;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let input = input
            .iter()
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        solve0(&input)
    }
}*/
r#macro::solution!(2022, 11, u128, {
    let input = input
        .split('\n')
        .filter(|x| !x.trim().is_empty())
        //.map(|x| x.to_owned())
        .collect::<Vec<_>>();
    solve0(&input)
});
