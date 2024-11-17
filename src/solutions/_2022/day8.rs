use crate::solutions::*;
use crate::Solution;

pub struct Solver;

fn solve1(input: Input) -> usize {
    let line_len = input[0].len();
    let tmatrix = input.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut vmatrix = vec![];
    let mut max = 0;
    for (y, &row) in tmatrix.iter().take(tmatrix.len() - 1).skip(1).enumerate() {
        let mut line = vec![];
        for (x, &tree) in row[1..line_len - 1].iter().enumerate() {
            let left = match row[..x + 1].iter().rev().position(|&x| x >= tree) {
                Some(pos) => pos + 1,
                None => x + 1,
            };
            let right = match row[x + 2..].iter().position(|&x| x >= tree) {
                Some(pos) => pos + 1,
                None => line_len - (x + 2),
            };
            let mut top = 1;
            for &i in tmatrix.iter().take(y).rev() {
                if i[x + 1] < tree {
                    top += 1;
                } else {
                    break;
                }
            }
            let mut bottom = 1;
            //println!("checking {tree}");
            for &i in tmatrix.iter().skip(y + 3) {
                //dbg!(i, i[x + 1]);
                if i[x + 1] < tree {
                    bottom += 1;
                } else {
                    break;
                }
            }
            let val = left * right * top * bottom;
            if val > max {
                max = val;
            }
            //println!("{left} {right} {top} {bottom}");
            line.push(val);
        }
        vmatrix.push(line);
    }
    //dbg!(vmatrix);
    max
}

fn solve0(input: Input) -> usize {
    let rows = input.len();
    let mut tallest_ltr_column: Vec<u8> = vec![];
    let mut matrix = vec![];
    let line_len = input[0].len();
    let mut line = vec![];
    for &char in input[0].as_bytes() {
        tallest_ltr_column.push(char);
        line.push(1);
    }
    matrix.push(line);
    for line in &input[1..rows - 1] {
        let mut flag_line = vec![];
        let mut tallest_in_row = None;
        for (idx, &char) in line.as_bytes().iter().enumerate() {
            if idx == 0 || idx == line_len - 1 {
                if idx == 0 {
                    tallest_in_row = Some(char);
                }
                flag_line.push(1);
            } else {
                let mut flag = 0;
                if tallest_ltr_column[idx] < char {
                    tallest_ltr_column[idx] = char;
                    flag = 1;
                }
                if tallest_in_row.unwrap() < char {
                    tallest_in_row = Some(char);
                    flag = 1;
                }
                flag_line.push(flag);
            }
        }
        matrix.push(flag_line);
    }
    let mut tallest_rtl_column: Vec<u8> = vec![];
    let mut flag_line = Vec::with_capacity(line_len);
    for &char in input[rows - 1].as_bytes().iter().rev() {
        tallest_rtl_column.push(char);
        flag_line.push(1);
    }
    matrix.push(flag_line);
    for (idx, line) in input[1..input.len() - 1].iter().rev().enumerate() {
        let mut tallest_in_row = None;
        let y = (rows - 2) - idx;
        for (idx, &char) in line.as_bytes().iter().rev().enumerate() {
            let x = (line_len - 1) - idx;
            //dbg!(x, y);
            /*if matrix[y][x] == 1 {
                if idx == 0 {
                    tallest_in_row = Some(char);
                }
                if tallest_rtl_column[idx] < char {
                    tallest_rtl_column[idx] = char;
                }
            } else {*/
            if idx == 0 {
                tallest_in_row = Some(char);
            }
            if tallest_rtl_column[idx] < char {
                tallest_rtl_column[idx] = char;
                matrix[y][x] = 1;
            }
            if tallest_in_row.unwrap() < char {
                tallest_in_row = Some(char);
                matrix[y][x] = 1;
            }
            //}
        }
    }
    matrix
        .iter()
        .fold(0, |prev, x| prev + x.iter().sum::<usize>())
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        (solve0(input), solve1(input))
    }
}
