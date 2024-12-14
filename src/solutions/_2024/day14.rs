use crate::solutions::*;
use regex::Regex;

fn solve(input: &str, t: i32, xmax: i32, ymax: i32) -> i32 {
    let re = Regex::new(r"\w+=(-?[\d]+),(-?[\d]+)").unwrap();
    let robots = input
        .lines()
        .flat_map(|r| {
            re.captures_iter(r)
                .map(|c| c.extract())
                .map(|(_, [x, y])| (i32_p(x), i32_p(y)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (q1, q2, q3, q4) = robots
        .chunks(2)
        .filter_map(|robot| {
            if let &[(px, py), (vx, vy)] = robot {
                let (mut px, mut py) = ((px + vx * t) % xmax, (py + vy * t) % ymax);
                if px < 0 {
                    px += xmax;
                }
                if py < 0 {
                    py += ymax;
                }
                Some((px, py))
                //(py != ymax / 2 && px != xmax / 2).then_some((px, py))
            } else {
                None
            }
        })
        .fold((0, 0, 0, 0), |(tl, tr, bl, br), (px, py)| {
            if px < xmax / 2 && py < ymax / 2 {
                (tl + 1, tr, bl, br)
            } else if px < xmax / 2 && py > ymax / 2 {
                (tl, tr, bl + 1, br)
            } else if px > xmax / 2 && py < ymax / 2 {
                (tl, tr + 1, bl, br)
            } else if px > xmax / 2 && py > ymax / 2 {
                (tl, tr, bl, br + 1)
            } else {
                (tl, tr, bl, br)
            }
        });
    q1 * q2 * q3 * q4
    //.collect::<Vec<_>>();
    //0
}

r#macro::solution!(2024, 14, i32, (solve(input, 100, 101, 103), 0));

#[test]
fn t() {
    //solve("p=2,4 v=2,-3", 5, 11, 7);
    //assert!(false);
    assert_eq!(
        solve(
            "p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3",
            100,
            11,
            7
        ),
        12
    );
}
