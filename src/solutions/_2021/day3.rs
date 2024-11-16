use crate::Solution;

pub struct Solver;

fn solve_0<'b>(numbers: &Vec<&'b String>, idx: usize, keep_less: bool) -> &'b String {
    //dbg!(numbers);
    if numbers.len() == 1 {
        return numbers[0];
    }
    let mut zeroes = Vec::with_capacity(numbers.len());
    let mut ones = Vec::with_capacity(numbers.len());
    for number in numbers {
        if number.as_bytes()[idx] == b'0' {
            zeroes.push(*number);
        } else {
            ones.push(*number);
        }
    }
    //dbg!(idx, &zeroes);
    //dbg!(idx, &ones);
    if keep_less {
        if zeroes.len() > ones.len() {
            return solve_0(&ones, idx + 1, keep_less);
        } else {
            return solve_0(&zeroes, idx + 1, keep_less);
        }
    } else if zeroes.len() > ones.len() {
        return solve_0(&zeroes, idx + 1, keep_less);
    } else {
        return solve_0(&ones, idx + 1, keep_less);
    }
}

impl Solution for Solver {
    type Answer = u32;

    fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {
        let input = input.iter().collect::<Vec<_>>();
        let oxygen = solve_0(&input, 0, false);
        let co2 = solve_0(&input, 0, true);
        let oxygen = u32::from_str_radix(oxygen, 2).unwrap();
        let co2 = u32::from_str_radix(co2, 2).unwrap();
        //println!("{} {}", oxygen, co2);
        /*let num =
            String::from_utf8_lossy(&counts.map(|x| if x > 0 { b'0' } else { b'1' })).to_string();
        let num = u8::from_str_radix(&num, 2).unwrap();*/
        (oxygen as u32 * co2 as u32, 0)
        //(0, 0)
        //(num * (!num & 0b0001111), 0)
        //(num * (!num & 0b0000111111111111) as u32, 0)
    }
}
