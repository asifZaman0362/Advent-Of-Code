use crate::solutions::*;
use regex::Regex;

struct LinearEquation {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_machine(info: &str) -> Vec<(LinearEquation, LinearEquation)> {
    let button_re = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let machines = button_re
        .captures_iter(info)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (u64_p(a), u64_p(b)))
        .collect::<Vec<_>>();
    let prizes = prize_re
        .captures_iter(info)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (u64_p(a), u64_p(b)))
        .collect::<Vec<_>>();
    machines
        .chunks(2)
        .zip(prizes.iter())
        .map(|(list, prize)| {
            (
                LinearEquation {
                    a: list[0].0,
                    b: list[1].0,
                    c: prize.0,
                },
                LinearEquation {
                    a: list[0].1,
                    b: list[1].1,
                    c: prize.1,
                },
            )
        })
        .collect::<Vec<_>>()
}

fn find_line_intersection(line1: &LinearEquation, line2: &LinearEquation) -> Option<(f64, f64)> {
    let det = (line1.a * line2.b) as i64 - (line2.a * line1.b) as i64;
    if det == 0 {
        None
    } else {
        let x = ((line2.b * line1.c) as i64 - (line1.b * line2.c) as i64) as f64 / det as f64;
        let y = ((line1.a * line2.c) as i64 - (line2.a * line1.c) as i64) as f64 / det as f64;
        Some((x, y))
    }
}

fn get_prize_cost((l1, l2): &(LinearEquation, LinearEquation)) -> Option<u64> {
    if l1.a * l2.b != l2.a * l1.b {
        if let Some((a, b)) = find_line_intersection(l1, l2) {
            if a.floor() == a && b.floor() == b {
                Some(3 * a as u64 + b as u64)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        if l1.c * l2.a == l2.c * l1.a {
            // infinite solutions
            println!("infinite");
            /*let (x, y) =
            for i in 0..100 {
            }*/
        }
        None
    }
}

fn solve(input: &str) -> (u64, u64) {
    let machines = parse_machine(input);
    let part1: u64 = machines.iter().filter_map(get_prize_cost).sum();
    let part2: u64 = machines
        .iter()
        .filter_map(|(l1, l2)| {
            get_prize_cost(&(
                LinearEquation {
                    c: l1.c + 10000000000000,
                    ..*l1
                },
                LinearEquation {
                    c: l2.c + 10000000000000,
                    ..*l2
                },
            ))
        })
        .sum();
    (part1, part2)
}

#[test]
fn test_machine_parse() {
    assert_eq!(
        solve(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        ),
        (480, 875318608908)
    );
}

r#macro::solution!(2024, 13, u64, solve(input));
