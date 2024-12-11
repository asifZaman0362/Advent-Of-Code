pub type Input<'a> = &'a str;

pub use std::cmp::Ordering;
pub use std::collections::{HashMap, HashSet};

pub type Grid<'a> = &'a [&'a [u8]];
pub type Pos = (isize, isize);

pub trait Number:
    std::ops::Add
    + std::ops::Sub
    + std::ops::Div
    + std::ops::Mul
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::DivAssign
    + std::ops::MulAssign
    + std::cmp::PartialOrd
    + std::cmp::PartialEq
    + Sized
{
}

impl Number for usize {}
impl Number for isize {}
impl Number for u32 {}
impl Number for i32 {}
impl Number for f32 {}
impl Number for u64 {}
impl Number for i64 {}
impl Number for u16 {}
impl Number for i16 {}
impl Number for u8 {}
impl Number for i8 {}

#[derive(Eq, PartialEq)]
pub struct Vec2<T: Number> {
    x: T,
    y: T,
}

impl<T: Number> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T: Number> std::ops::Add<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Add>::Output: Number,
{
    type Output = Vec2<<T as std::ops::Add>::Output>;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from((self.x + rhs.x, self.y + rhs.y))
    }
}

impl<T: Number> std::ops::Sub<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Sub>::Output: Number,
{
    type Output = Vec2<<T as std::ops::Sub>::Output>;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from((self.x - rhs.x, self.y - rhs.y))
    }
}

impl<T: Number> std::ops::Mul<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Mul>::Output: Number,
{
    type Output = Vec2<<T as std::ops::Mul>::Output>;
    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from((self.x * rhs.x, self.y * rhs.y))
    }
}

impl<T: Number> std::ops::Div<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Div>::Output: Number,
{
    type Output = Vec2<<T as std::ops::Div>::Output>;
    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::from((self.x / rhs.x, self.y / rhs.y))
    }
}

impl<T: Number> std::ops::AddAssign<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Add>::Output: Number,
{
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Number> std::ops::SubAssign<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Sub>::Output: Number,
{
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Number> std::ops::MulAssign<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Mul>::Output: Number,
{
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Number> std::ops::DivAssign<Vec2<T>> for Vec2<T>
where
    <T as std::ops::Div>::Output: Number,
{
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Number + Copy> std::ops::DivAssign<T> for Vec2<T>
where
    <T as std::ops::Div>::Output: Number,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Number + Copy> std::ops::Div<T> for Vec2<T>
where
    <T as std::ops::Div>::Output: Number,
{
    type Output = Vec2<<T as std::ops::Div>::Output>;
    fn div(self, rhs: T) -> Self::Output {
        Vec2::from((self.x / rhs, self.y / rhs))
    }
}

pub trait Solution {
    type Answer: std::fmt::Display + std::fmt::Debug;
    fn solve(input: Input) -> (Self::Answer, Self::Answer);
    fn date() -> (u16, u8);
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

pub fn asst<T: PartialEq + std::fmt::Debug>(a: T, b: T) {
    assert_eq!(a, b);
}

pub mod _2015;
pub mod _2020;
pub mod _2021;
pub mod _2022;
pub mod _2024;
