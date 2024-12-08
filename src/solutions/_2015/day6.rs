use crate::solutions::*;

pub struct Solver;

enum Op {
    On,
    Off,
    Toggle,
}

fn update_lights(
    (sx, sy): (usize, usize),
    (ex, ey): (usize, usize),
    op: Op,
    lights: &mut [[u8; 1000]],
    lights2: &mut [[u8; 1000]],
) {
    for y in sy..=ey {
        for x in sx..=ex {
            lights2[y][x] = match op {
                Op::On => lights2[y][x] + 1,
                Op::Off => {
                    if lights2[y][x] > 0 {
                        lights2[y][x] - 1
                    } else {
                        0
                    }
                }
                Op::Toggle => lights2[y][x] + 2,
            };
            lights[y][x] = match op {
                Op::On => 1,
                Op::Off => 0,
                Op::Toggle => lights[y][x] ^ 1,
            }
        }
    }
}

fn count_on(lights: &[[u8; 1000]], lights2: &[[u8; 1000]]) -> (usize, usize) {
    let mut count1: usize = 0;
    let mut count2: usize = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            count1 += lights[y][x] as usize;
            count2 += lights2[y][x] as usize;
        }
    }
    (count1, count2)
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let mut lights = [[0u8; 1000]; 1000];
        let mut lights2 = [[0u8; 1000]; 1000];
        for instruction in input {
            let (op, rest) = if let Some(rest) = instruction.strip_prefix("turn on ") {
                (Op::On, rest)
            } else if let Some(rest) = instruction.strip_prefix("turn off ") {
                (Op::Off, rest)
            } else if let Some(rest) = instruction.strip_prefix("toggle ") {
                (Op::Toggle, rest)
            } else {
                panic!("invalid instruction: {instruction}");
            };
            if let Some((start, end)) = rest.split_once(" through ") {
                let start = start
                    .split_once(",")
                    .map(|(x, y)| {
                        //println!("{x} {y}");
                        (usize_p(x), usize_p(y))
                    })
                    .unwrap();
                let end = end
                    .split_once(",")
                    .map(|(x, y)| {
                        //println!("{x} {y}");
                        (usize_p(x), usize_p(y))
                    })
                    .unwrap();
                update_lights(start, end, op, &mut lights, &mut lights2);
            }
        }
        count_on(&lights, &lights2)
    }
}
