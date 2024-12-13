use crate::solutions::*;

fn find_area(
    chars: &[Vec<char>],
    x: usize,
    y: usize,
    seen: &mut HashSet<Vec2<usize>>,
    regions: &mut HashMap<(usize, usize), usize>,
    region: usize,
) -> (usize, usize) {
    help(chars[y][x], chars, (x, y), seen, regions, region).unwrap()
}

fn calc_sides(
    crops: &HashMap<(usize, usize), char>,
    sx: usize,
    sy: usize,
    regions: &HashMap<(usize, usize), usize>,
    side_list: &mut [usize],
) -> usize {
    let mut flags = HashMap::<(usize, usize), (bool, bool, bool, bool)>::new();
    for y in 0..sy {
        for x in 0..sx {
            let current = *crops.get(&(x, y)).unwrap();
            let (mut cur_top, mut cur_bot, mut cur_left, mut cur_right) =
                (false, false, false, false);
            let (lt, lb, _, _) = flags
                .get(&(x.wrapping_sub(1), y))
                .copied()
                .unwrap_or((false, false, false, false));
            let (_, _, tl, tr) = flags
                .get(&(x, y.wrapping_sub(1)))
                .copied()
                .unwrap_or((false, false, false, false));
            let left = crops.get(&(x.wrapping_sub(1), y)).copied().unwrap_or('*');
            let top = crops.get(&(x, y.wrapping_sub(1))).copied().unwrap_or('*');
            let right = crops.get(&(x + 1, y)).copied().unwrap_or('*');
            let bottom = crops.get(&(x, y + 1)).copied().unwrap_or('*');
            let mut sides = 0;
            if left != current {
                cur_left = true;
                sides += if top == current && tl { 0 } else { 1 };
            }
            if top != current {
                cur_top = true;
                sides += if left == current && lt { 0 } else { 1 };
            }
            if bottom != current {
                cur_bot = true;
                sides += if left == current && lb { 0 } else { 1 };
            }
            if right != current {
                cur_right = true;
                sides += if top == current && tr { 0 } else { 1 };
            }
            flags.insert((x, y), (cur_top, cur_bot, cur_left, cur_right));
            if sides > 0 {
                side_list[*regions.get(&(x, y)).unwrap()] += sides;
            }
        }
    }
    0
}

fn help(
    crop: char,
    chars: &[Vec<char>],
    (x, y): (usize, usize),
    seen: &mut HashSet<Vec2<usize>>,
    regions: &mut HashMap<(usize, usize), usize>,
    region: usize,
) -> Option<(usize, usize)> {
    if chars[y][x] != crop {
        None
    } else if seen.contains(&(x, y).into()) {
        return Some((0, 0));
    } else {
        let x_max = chars[0].len();
        let y_max = chars.len();
        regions.insert((x, y), region);
        seen.insert((x, y).into());
        let mut total_peri = 4;
        let mut tot_area = 1;
        if x > 0 {
            let right = help(crop, chars, (x - 1, y), seen, regions, region);
            if let Some((area, peri)) = right {
                total_peri = (total_peri - 1) + peri;
                tot_area += area;
            }
        }
        if x < x_max - 1 {
            let left = help(crop, chars, (x + 1, y), seen, regions, region);
            if let Some((area, peri)) = left {
                total_peri = (total_peri - 1) + peri;
                tot_area += area;
            }
        }
        if y > 0 {
            let top = help(crop, chars, (x, y - 1), seen, regions, region);
            if let Some((area, peri)) = top {
                total_peri = (total_peri - 1) + peri;
                tot_area += area;
            }
        }
        if y < y_max - 1 {
            let bottom = help(crop, chars, (x, y + 1), seen, regions, region);
            if let Some((area, peri)) = bottom {
                total_peri = (total_peri - 1) + peri;
                tot_area += area;
            }
        }
        Some((tot_area, total_peri))
    }
}

fn solve(input: &str) -> (usize, usize) {
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut cost1 = 0;
    let mut seen = HashSet::<Vec2<usize>>::new();
    let mut charmap = HashMap::new();
    let mut region = 0;
    let mut sides = vec![];
    let mut areas = vec![];
    let mut regions = vec![];
    let mut regionmap = HashMap::new();
    for (y, line) in chars.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            charmap.insert((x, y), c);
            if !seen.contains(&(x, y).into()) {
                let (area, peri) = find_area(&chars, x, y, &mut seen, &mut regionmap, region);
                regions.push(c);
                cost1 += area * peri;
                sides.push(0);
                areas.push(area);
                region += 1;
            }
        }
    }
    calc_sides(
        &charmap,
        chars[0].len(),
        chars.len(),
        &regionmap,
        &mut sides,
    );
    (
        cost1,
        areas
            .iter()
            .zip(sides.iter())
            .map(|(area, side)| area * side)
            .sum(),
    )
}

r#macro::solution!(2024, 12, usize, solve(input));

#[test]
fn t() {
    assert_eq!(
        solve(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        ),
        (1930, 1206)
    );
}
