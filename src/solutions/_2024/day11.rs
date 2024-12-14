use crate::solutions::*;

use r#macro::{solution, test_sol};

fn split(stone: u64) -> Option<(u64, u64)> {
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let mid = digits / 2;
        let right = stone % 10u64.pow(mid);
        let left = stone / 10u64.pow(mid);
        Some((left, right))
    } else {
        None
    }
}

fn blink_stone(stone: u64, count: u8, cache: &mut HashMap<(u64, u8), usize>) -> usize {
    if let Some(val) = cache.get(&(stone, count)) {
        *val
    } else {
        let val = if count == 1 {
            if stone == 0 || stone == 1 {
                1
            } else if (stone.ilog10() + 1) % 2 == 0 {
                2
            } else {
                1
            }
        } else if stone == 0 {
            blink_stone(1, count - 1, cache)
        } else if let Some((left, right)) = split(stone) {
            blink_stone(left, count - 1, cache) + blink_stone(right, count - 1, cache)
        } else {
            blink_stone(stone * 2024, count - 1, cache)
        };
        cache.insert((stone, count), val);
        val
    }
}

fn blink(stones: &[u64]) -> (usize, usize) {
    let mut cache = HashMap::<(u64, u8), usize>::new();
    let (mut part1, mut part2) = (0, 0);
    for &stone in stones {
        part1 += blink_stone(stone, 25, &mut cache);
    }
    for &stone in stones {
        part2 += blink_stone(stone, 75, &mut cache);
    }
    (part1, part2)
}

#[test]
fn sol() {
    test_sol!("125 17", 55312, 65601038650482);
}

solution!(2024, 11, usize, {
    let stones = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    blink(&stones)
});
