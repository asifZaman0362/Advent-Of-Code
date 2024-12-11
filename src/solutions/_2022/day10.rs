use crate::solutions::*;

//pub struct Solver;

fn draw(s: i32, xpos: i32) {
    if (s - 1..s + 2).contains(&xpos) {
        print!("#");
    } else {
        print!(".");
    }
    if (xpos + 1) % 40 == 0 {
        println!();
    }
}

fn inc(x: i32) -> i32 {
    if x == 39 {
        0
    } else {
        x + 1
    }
}

fn solve0(input: Input) -> i32 {
    let mut X = 1;
    let mut history = vec![X];
    let mut xpos = 0;
    draw(X, xpos);
    xpos = inc(xpos);
    for line in input.split('\n') {
        if line == "noop" {
            draw(X, xpos);
            history.push(X);
            xpos = inc(xpos);
        } else {
            let (_, num) = line.split_once(" ").unwrap();
            draw(X, xpos);
            xpos = inc(xpos);
            history.push(X);
            X += num.parse::<i32>().unwrap();
            draw(X, xpos);
            history.push(X);
            xpos = inc(xpos);
        }
    }
    history
        .iter()
        .enumerate()
        .filter(|(idx, val)| {
            //println!("{} {}", idx, val);
            (*idx as i32 - 19) % 40 == 0
        })
        .fold(0, |prev, (idx, val)| prev + (idx + 1) as i32 * val)
}

r#macro::solution!(2022, 10, i32, (solve0(input), -1));

/*impl Solution for Solver {
    type Answer = i32;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        (solve0(input), -1)
    }
}*/
