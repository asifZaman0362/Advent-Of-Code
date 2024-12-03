pub type Input<'a> = &'a Vec<String>;

pub trait Solution {
    type Answer: std::fmt::Display + std::fmt::Debug;
    fn solve(input: Input) -> (Self::Answer, Self::Answer);
}

pub fn u64_p(x: &str) -> u64 {
    x.parse::<u64>().unwrap()
}

pub fn i64_p(x: &str) -> i64 {
    x.parse::<i64>().unwrap()
}

pub fn f64_p(x: &str) -> f64 {
    x.parse::<f64>().unwrap()
}

pub fn u32_p(x: &str) -> u32 {
    x.parse::<u32>().unwrap()
}

pub fn i32_p(x: &str) -> i32 {
    x.parse::<i32>().unwrap()
}

pub fn f32_p(x: &str) -> f32 {
    x.parse::<f32>().unwrap()
}

pub mod _2020;
pub mod _2021;
pub mod _2022;
pub mod _2024;
