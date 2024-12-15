use crate::solutions::*;

fn solve(input: &str) -> usize {
    let div = input.lines().position(|l| l.is_empty()).expect(
        "input doesn't contain a break between the initial state and the movement sequence",
    );
    let mut state = HashMap::<Vec2<usize>, char>::new();
    let (mut sx, mut sy) = (0, 0);
    for (y, r) in input.lines().take(div).enumerate() {
        for (x, c) in r.chars().enumerate() {
            state.insert((x, y).into(), c);
            if c == '@' {
                sx = x;
                sy = y;
            }
        }
    }
    let mut pos: Vec2<usize> = (sx, sy).into();
    for dir in input.lines().skip(div + 1).flat_map(|l| l.chars()) {
        let motion: Vec2<isize> = match dir {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        }
        .into();
        let new: Vec2<usize> = pos + motion;
        if let Some(&adjacent) = state.get(&new) {
            if adjacent == '.' {
                state.insert(new, '@');
                state.insert(pos, '.');
                pos = new;
            } else if adjacent == 'O' {
                let mut next = new + motion;
                while let Some(search) = state.get_mut(&next) {
                    if *search == '.' {
                        *search = 'O';
                        state.insert(new, '@');
                        state.insert(pos, '.');
                        pos = new;
                        break;
                    } else if *search == '#' {
                        break;
                    } else {
                        next += motion;
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for (pos, &c) in state.iter() {
        if c == 'O' {
            sum += pos.y * 100 + pos.x;
        }
    }
    sum
}

fn can_push(
    blocks: &[&mut [char]],
    pos: Vec2<usize>,
    dir: Vec2<isize>,
    checked: &mut HashSet<Vec2<usize>>,
) -> bool {
    let next = pos + dir;
    let mut adj = pos;
    let block = blocks[pos.y][pos.x];
    adj.x = if block == '[' { adj.x + 1 } else { adj.x - 1 };
    checked.contains(&pos) || {
        checked.insert(pos);
        if let Some(&n) = blocks.get(next.y).and_then(|s| s.get(next.x)) {
            if n == '.' {
                checked.contains(&adj) || can_push(blocks, adj, dir, checked)
            } else {
                n != '#'
                    && can_push(blocks, next, dir, checked)
                    && can_push(blocks, adj, dir, checked)
            }
        } else {
            false
        }
    }
}

fn push(
    blocks: &mut [&mut [char]],
    pos: Vec2<usize>,
    dir: Vec2<isize>,
    moved: &mut HashSet<Vec2<usize>>,
) {
    if moved.contains(&pos) {
        return;
    }
    if pos.y >= blocks.len() - 1 || pos.x >= blocks[0].len() - 1 {
        return;
    }
    let block = blocks[pos.y][pos.x];
    if block == '.' || block == '#' {
        return;
    }
    let next = pos + dir;
    if block == '[' {
        moved.insert(pos);
        push(blocks, next, dir, moved);
        blocks[next.y][next.x] = block;
        blocks[pos.y][pos.x] = '.';
        push(blocks, (pos.x + 1, pos.y).into(), dir, moved);
    }
    if block == ']' {
        moved.insert(pos);
        push(blocks, next, dir, moved);
        blocks[next.y][next.x] = block;
        blocks[pos.y][pos.x] = '.';
        push(blocks, (pos.x - 1, pos.y).into(), dir, moved);
    }
}

fn try_push(state: &mut [&mut [char]], pos: Vec2<usize>, dir: Vec2<isize>) -> bool {
    if dir.y.abs() == 1 {
        let mut checked = HashSet::new();
        let mut moved = HashSet::new();
        if can_push(state, pos, dir, &mut checked) {
            push(state, pos, dir, &mut moved);
            return true;
        }
        false
    } else {
        let mut next = pos + dir;
        while let Some(&search) = state[next.y].get(next.x) {
            if search == '.' {
                let rev = Vec2 {
                    x: -dir.x,
                    y: -dir.y,
                };
                while next != pos {
                    let p = next + rev;
                    state[next.y][next.x] = state[p.y][p.x];
                    next += rev;
                }
                return true;
            } else if search == '#' {
                return false;
            } else {
                next += dir;
            }
        }
        false
    }
}

fn part2(input: &str) -> usize {
    let div = input.lines().position(|l| l.is_empty()).expect(
        "input doesn't contain a break between the initial state and the movement sequence",
    );
    let mut pos: Vec2<usize> = (0, 0).into();
    let mut chars = input
        .lines()
        .take(div)
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => {
                        pos = (x * 2, y).into();
                        ['@', '.']
                    }
                    c => unreachable!("bruh! whats this at {x} {y}: '{c}'"),
                })
                .collect::<Vec<_>>()
                .leak()
        })
        .collect::<Vec<_>>();
    for dir in input.lines().skip(div + 1).flat_map(|l| l.chars()) {
        let motion: Vec2<isize> = match dir {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => unreachable!(),
        }
        .into();
        let new: Vec2<usize> = pos + motion;
        if let Some(&adjacent) = chars.get(new.y).and_then(|l| l.get(new.x)) {
            if adjacent == '.' || ("[]".contains(adjacent) && try_push(&mut chars, new, motion)) {
                chars[new.y][new.x] = '@';
                chars[pos.y][pos.x] = '.';
                pos = new;
            }
        }
    }
    let mut sum = 0;
    let (mut x, mut y) = (0, 0);
    while y < chars.len() {
        while x < chars[0].len() {
            if chars[y][x] == '[' {
                sum += y * 100 + x;
                x += 2;
            } else {
                x += 1;
            }
        }
        y += 1;
        x = 0;
    }
    sum
}

fn sol(input: &str) -> (usize, usize) {
    (solve(input), part2(input))
}

r#macro::solution!(2024, 15, usize, sol(input));

#[test]
fn t() {
    assert_eq!(
        sol("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"),
        (10092, 9021)
    );
    //assert!(false);
}
