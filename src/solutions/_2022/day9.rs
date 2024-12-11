use crate::solutions::*;
use std::collections::HashSet;

//pub struct Solver;

type Pos = (i32, i32);

fn keep_up(tail: &Pos, head: &Pos) -> Pos {
    let mut t: (i32, i32) = *tail;
    if (head.0 - tail.0).abs() >= 2 {
        t.0 += (head.0 - tail.0).signum();
        if (head.1 - tail.1).abs() >= 1 {
            t.1 += (head.1 - tail.1).signum();
        }
        t
    } else if (head.1 - tail.1).abs() >= 2 {
        t.1 += (head.1 - tail.1).signum();
        if (head.0 - tail.0).abs() >= 1 {
            t.0 += (head.0 - tail.0).signum();
        }
        t
    } else {
        *tail
    }
}

fn _move(
    motion: (i32, i32),
    steps: u32,
    head: &mut Pos,
    tail: &mut [Pos],
    list: &mut HashSet<Pos>,
) {
    for _ in 0..steps {
        head.0 += motion.0;
        head.1 += motion.1;
        tail[0] = keep_up(&tail[0], head);
        for i in 1..tail.len() {
            tail[i] = keep_up(&tail[i], &tail[i - 1]);
        }
        list.insert(*tail.last().unwrap());
    }
}

fn make_motion(
    motion: &str,
    head: &mut (i32, i32),
    tail: &mut [(i32, i32)],
    pos: &mut HashSet<(i32, i32)>,
) {
    let (dir, steps) = motion.split_once(" ").unwrap();
    let steps = steps.parse::<u32>().unwrap();
    match (dir, steps) {
        ("L", steps) => {
            _move((-1, 0), steps, head, tail, pos);
        }
        ("R", steps) => {
            _move((1, 0), steps, head, tail, pos);
        }
        ("U", steps) => {
            _move((0, 1), steps, head, tail, pos);
        }
        ("D", steps) => {
            _move((0, -1), steps, head, tail, pos);
        }
        _ => unreachable!("bad input!"),
    }
}

fn solve0(input: Input) -> usize {
    let mut head = (0, 0);
    let mut tail = [(0, 0); 1];
    let mut tail_pos = HashSet::<(i32, i32)>::new();
    tail_pos.insert((0, 0));
    for motion in input.lines() {
        make_motion(motion, &mut head, &mut tail, &mut tail_pos);
    }
    //dbg!(&tail_pos);
    //let mut matrix = [[0; 32]; [32]];
    /*for i in 0..8 {
        for j in 0..8 {
            if tail_pos.contains(&(j, 7 - i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }*/
    tail_pos.len()
}

fn solve1(input: Input) -> usize {
    let mut head = (0, 0);
    let mut tail = [(0, 0); 9];
    let mut tail_pos = HashSet::<(i32, i32)>::new();
    tail_pos.insert((0, 0));
    for motion in input.lines() {
        make_motion(motion, &mut head, &mut tail, &mut tail_pos);
    }
    //dbg!(&tail_pos);
    //let mut matrix = [[0; 32]; [32]];
    /*for i in 0..8 {
        for j in 0..8 {
            if tail_pos.contains(&(j, 7 - i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }*/
    tail_pos.len()
}

/*impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        (solve0(input), solve1(input))
    }
}*/
r#macro::solution!(2022, 9, usize, (solve0(input), solve1(input)));
