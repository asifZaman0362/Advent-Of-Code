pub type Input<'a> = &'a Vec<String>;

pub use std::cmp::Ordering;
pub use std::collections::{HashMap, HashSet};

pub type Grid<'a> = &'a [&'a [u8]];
pub type Pos = (isize, isize);

pub trait Solution {
    type Answer: std::fmt::Display + std::fmt::Debug;
    fn solve(input: Input) -> (Self::Answer, Self::Answer);
}

pub fn u64_p(x: &str) -> u64 {
    x.parse::<u64>().unwrap()
}

pub fn usize_p(x: &str) -> usize {
    x.parse::<usize>().unwrap()
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

pub fn sorted<T: std::cmp::Ord>(vec: &mut Vec<T>) -> impl std::iter::Iterator<Item = &mut T> {
    vec.sort();
    vec.iter_mut()
}

pub fn next(x: usize, dx: isize, i: usize) -> Option<usize> {
    if dx.signum() == -1 {
        let dx: usize = (-dx).try_into().unwrap();
        x.checked_sub(dx * (i + 1))
    } else {
        let dx: usize = (dx).try_into().unwrap();
        x.checked_add(dx * (i + 1))
    }
}

pub fn next_mut(x: &mut usize, dx: isize) -> Option<usize> {
    (if dx.signum() == -1 {
        let dx: usize = (-dx).try_into().unwrap();
        x.checked_sub(dx)
    } else {
        let dx: usize = (dx).try_into().unwrap();
        x.checked_add(dx)
    })
    .inspect(|new| {
        *x = *new;
    })
}

pub trait Sorted<T: std::cmp::Ord> {
    fn sorted<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a T>
    where
        T: 'a;
    fn sorted_mut<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a mut T>
    where
        T: 'a;
}

impl<T: std::cmp::Ord> Sorted<T> for Vec<T> {
    fn sorted<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.sort();
        self.iter()
    }
    fn sorted_mut<'a>(&'a mut self) -> impl std::iter::Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.sort();
        self.iter_mut()
    }
}

pub mod _2015;
pub mod _2020;
pub mod _2021;
pub mod _2022;
pub mod _2024;
