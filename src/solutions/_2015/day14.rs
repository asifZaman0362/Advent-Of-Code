use crate::solutions::*;
use regex;

#[test]
fn test_sol() {
    assert_eq!(
        solve(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
            1000.0
        ),
        (1120, 0)
    );
}

fn solve(input: &str, time: f32) -> (u32, u32) {
    let ex = regex::Regex::new(r"(\w+)\s.*\s([\d]+)\s.*\s([\d]+)\s.*\s([\d]+).*").unwrap();
    let info = input
        .lines()
        .map(|line| {
            let cap = ex.captures(line).unwrap();
            (
                cap.get(1).unwrap().as_str(),
                u32_p(cap.get(2).unwrap().as_str()),
                u32_p(cap.get(3).unwrap().as_str()),
                u32_p(cap.get(4).unwrap().as_str()),
            )
        })
        .collect::<Vec<_>>();
    let part1 = info.iter().fold(0, |prev, (name, speed, fly, rest)| {
        let tot = fly + rest;
        let tot_fly = (time / tot as f32).ceil() as u32;
        let fl = speed * fly * tot_fly;
        fl.max(prev)
    });
    let mut points = info.iter().map(|_| 0).collect::<Vec<u32>>();
    //for i in 0..time {}
    (part1, 0)
}

r#macro::solution!(2015, 14, u32, solve(input, 2503.0));
