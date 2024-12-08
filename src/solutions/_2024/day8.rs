use crate::solutions::*;

pub struct Solver;

fn next(a: &mut Pos, diff: &Pos, max: Pos) -> Option<Pos> {
    let res = (a.0 - diff.0, a.1 - diff.1);
    *a = res;
    (res.0 >= 0 && res.0 < max.0 && res.1 >= 0 && res.1 < max.1).then_some(res)
}

fn compute_antinode_pos(a: &Pos, b: &Pos, xmax: isize, ymax: isize) -> [Option<Pos>; 2] {
    let diff = (a.0 - b.0, a.1 - b.1);
    let first = (a.0 + diff.0, a.1 + diff.1);
    let second = (b.0 - diff.0, b.1 - diff.1);
    let first = (first.0 >= 0 && first.0 < xmax && first.1 >= 0 && first.1 < ymax).then_some(first);
    let second =
        (second.0 >= 0 && second.0 < xmax && second.1 >= 0 && second.1 < ymax).then_some(second);
    [first, second]
}

fn compute_antinodes2(a: &Pos, b: &Pos, xmax: isize, ymax: isize) -> HashSet<Pos> {
    let mut poss = HashSet::from([*a, *b]);
    let diff = (a.0 - b.0, a.1 - b.1);
    let mut i = *a;
    while let Some(n) = next(&mut i, &diff, (xmax, ymax)) {
        poss.insert(n);
    }
    let mut i = *b;
    let diff = (-diff.0, -diff.1);
    while let Some(n) = next(&mut i, &diff, (xmax, ymax)) {
        poss.insert(n);
    }
    poss
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let mut map = HashMap::<char, Vec<Pos>>::new();
        let mut antinodes = HashSet::new();
        let mut antinodes_2 = HashSet::<Pos>::new();
        let (xmax, ymax) = (input.len() as isize, input[0].len() as isize);
        for (y, line) in input.iter().enumerate() {
            for (x, freq) in line.chars().enumerate() {
                if freq.is_alphanumeric() {
                    let (x, y) = (x as isize, y as isize);
                    if let Some(entry) = map.get_mut(&freq) {
                        for other in &*entry {
                            compute_antinode_pos(&(x, y), other, xmax, ymax).map(|x| {
                                if let Some(x) = x {
                                    antinodes.insert(x);
                                }
                            });
                            antinodes_2
                                .extend(compute_antinodes2(&(x, y), other, xmax, ymax).iter());
                        }
                        entry.push((x, y));
                    } else {
                        map.insert(freq, vec![(x, y)]);
                    }
                }
            }
        }
        (antinodes.len(), antinodes_2.len())
    }
}
