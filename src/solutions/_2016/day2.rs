use crate::solutions::*;

fn solve(input: &str) -> (String, String) {
    let mut pos: Vec2<usize> = (1, 1).into();
    let mut code = "".to_string();
    let buttons = ["123", "456", "789"]
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let _ = input
        .lines()
        .map(|line| {
            line.chars().for_each(|c| match c {
                'U' => {
                    if pos.y > 0 {
                        pos.y -= 1;
                    }
                }
                'D' => {
                    if pos.y < 2 {
                        pos.y += 1
                    }
                }
                'L' => {
                    if pos.x > 0 {
                        pos.x -= 1;
                    }
                }
                'R' => {
                    if pos.x < 2 {
                        pos.x += 1
                    }
                }
                _ => unreachable!(),
            });
            code.push(buttons[pos.y][pos.x]);
        })
        .collect::<Vec<_>>();
    (code, solve2(input))
}

fn solve2(input: &str) -> String {
    let mut pos: Vec2<usize> = (0, 2).into();
    let mut code = "".to_string();
    let buttons = ["  1  ", " 234 ", "56789", " ABC ", "  D  "]
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let _ = input
        .lines()
        .map(|line| {
            line.chars().for_each(|c| match c {
                'U' => {
                    if pos.y > 0 && !buttons[pos.y - 1][pos.x].is_ascii_whitespace() {
                        pos.y -= 1;
                    }
                }
                'D' => {
                    if pos.y < 4 && !buttons[pos.y + 1][pos.x].is_ascii_whitespace() {
                        pos.y += 1
                    }
                }
                'L' => {
                    if pos.x > 0 && !buttons[pos.y][pos.x - 1].is_ascii_whitespace() {
                        pos.x -= 1;
                    }
                }
                'R' => {
                    if pos.x < 4 && !buttons[pos.y][pos.x + 1].is_ascii_whitespace() {
                        pos.x += 1
                    }
                }
                _ => unreachable!(),
            });
            code.push(buttons[pos.y][pos.x]);
        })
        .collect::<Vec<_>>();
    code
}

r#macro::solution!(2016, 2, String, solve(input));

#[test]
fn t() {
    assert_eq!(
        solve(
            "ULL
RRDDD
LURDL
UUUUD"
        ),
        ("1985".to_string(), "5DB3".to_string())
    );
}
