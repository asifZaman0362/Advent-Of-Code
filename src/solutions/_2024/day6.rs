use crate::solutions::*;

//pub struct Solver;

fn next_char(input: Grid, (x, y): Pos, (dx, dy): (isize, isize)) -> Option<(char, Pos)> {
    next(x, dx, 0).and_then(|x| {
        next(y, dy, 0).and_then(|y| {
            input
                .get(y)
                .and_then(|line| line.get(x))
                .map(|c| (*c as char, (x, y)))
            //.inspect(|(c, ..)| println!("{x} {y} {c}"))
        })
    })
}

fn rot((x, y): (isize, isize)) -> (isize, isize) {
    match (x, y) {
        (0, 1) => (-1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn calculate_path(input: Grid, start: Pos) -> (Set, usize) {
    let (mut dx, mut dy) = (0, -1);
    let mut pos = start;
    //println!("starting at: {start:?}");
    let mut walkable = HashSet::from([start]);
    let mut loops = HashSet::new();
    while let Some((char, new_pos)) = next_char(input, pos, (dx, dy)) {
        if char == '#' {
            (dx, dy) = rot((dx, dy));
        } else {
            if check_loop(input, start, new_pos, (0, -1)) {
                loops.insert(new_pos);
            }
            pos = new_pos;
            walkable.insert(pos);
        }
    }
    (walkable, loops.len())
}

type Pos = (usize, usize);
type Set = HashSet<Pos>;
type Dir = (isize, isize);

fn check_loop(input: Grid, start: Pos, obstacle: Pos, dir: Dir) -> bool {
    //println!("checking loop for {obstacle:?}");
    let mut hit_obstacles = HashSet::new();
    let mut pos = start;
    let mut dir = dir;
    while let Some((char, new_pos)) = next_char(input, pos, dir) {
        if char == '#' || new_pos == obstacle {
            if hit_obstacles.contains(&(new_pos, dir)) {
                return true;
            } else {
                hit_obstacles.insert((new_pos, dir));
                dir = rot(dir);
            }
        } else {
            pos = new_pos;
        }
    }
    false
}

fn solve0(input: Grid) -> (usize, usize) {
    let mut start = (0, 0);
    'outer: for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &b'^' {
                start = (x, y);
                break 'outer;
            }
        }
    }
    let (walkable, loops) = calculate_path(input, start);
    (walkable.len(), loops)
}

/*impl Solution for Solver {
type Answer = usize;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 6, usize, {
    let input = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    solve0(&input)
});
