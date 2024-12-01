pub type Input<'a> = &'a Vec<String>;

pub trait Solution {
    type Answer: std::fmt::Display + std::fmt::Debug;
    fn solve(input: Input) -> (Self::Answer, Self::Answer);
}

pub mod _2020;
pub mod _2021;
pub mod _2022;
pub mod _2024;
