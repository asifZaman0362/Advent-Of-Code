use crate::solutions::*;
use std::collections::VecDeque;

pub struct Solver;

enum Token {
    Old,
    Value(usize),
    Operation(Operation),
}

enum Operation {
    Add,
    Multiply,
}

struct Monkey {
    items: VecDeque<usize>,
    tokens: Vec<Token>,
    divisibility: usize,
    decision: [usize; 2],
    inspection_count: usize,
}

fn evaluate(token: &Token, stress: usize) -> usize {
    match token {
        Token::Old => stress,
        Token::Value(val) => *val,
        _ => unreachable!("bad code!"),
    }
}

impl Monkey {
    fn new(block: &[String]) -> Self {
        let items = block[0]
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
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
                x => Token::Value(x.parse::<usize>().unwrap()),
            })
            .collect();
        let divisibility = block[2]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let true_branch = block[3]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let false_branch = block[4]
            .split(' ')
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
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

    fn inspect(&mut self) -> Option<(usize, usize)> {
        self.items.pop_front().map(|x| {
            self.inspection_count += 1;
            let (stress, next) = self.test(x);
            (stress, self.decision[next])
        })
    }

    fn test(&self, stress: usize) -> (usize, usize) {
        let stress = match self.tokens[1] {
            Token::Operation(Operation::Multiply) => stress * evaluate(&self.tokens[2], stress),
            Token::Operation(Operation::Add) => stress + evaluate(&self.tokens[2], stress),
            _ => unreachable!("bad code!"),
        } / 3;
        let next = match stress % self.divisibility {
            0 => 0,
            _ => 1,
        };
        (stress, next)
    }
}

fn simulate_round(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        let mut transfers = vec![];
        while let Some((item, next)) = monkeys[i].inspect() {
            transfers.push((item, next));
        }
        for (item, next) in transfers {
            monkeys[next].items.push_back(item);
        }
    }
}

fn solve0(input: Input) -> usize {
    let mut monkeys = vec![];
    for block in input.windows(6).step_by(6) {
        monkeys.push(Monkey::new(&block[1..]))
    }
    for _ in 0..20 {
        simulate_round(&mut monkeys);
    }
    monkeys.sort_by(|x, y| y.inspection_count.cmp(&x.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        (solve0(input), 0)
    }
}
