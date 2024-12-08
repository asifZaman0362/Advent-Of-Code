use crate::solutions::*;

pub struct Solver;

fn meets_req(req: &HashMap<&str, u64>, item: &str, count: u64) -> bool {
    let value = *req.get(&item).unwrap();
    match item {
        "cats" | "trees" => count > value,
        "goldfish" | "pomeranians" => count < value,
        _ => count == value,
    }
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let requirements = HashMap::from([
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]);
        let mut sue_part1 = None;
        let mut sue_part2 = None;
        for aunt in input {
            let (sue, possessions) = aunt.split_once(": ").unwrap();
            let sue = sue.split_once(" ").unwrap().1;
            let mut part1 = true;
            let mut part2 = true;
            //aunt_info.push(
            for (item, count) in possessions.split(", ").map(|pos| {
                let (kind, count) = pos.split_once(": ").unwrap();
                let count = u64_p(count);
                (kind, count)
            }) {
                if requirements.get(&item).unwrap() != &count {
                    part1 = false;
                }
                if !meets_req(&requirements, item, count) {
                    part2 = false;
                    //continue 'outer;
                }
            }
            if sue_part1.is_none() && part1 {
                sue_part1 = Some(usize_p(sue));
            }
            if sue_part2.is_none() && part2 {
                sue_part2 = Some(usize_p(sue));
            }
            if let Some(sue1) = sue_part1 {
                if let Some(sue2) = sue_part2 {
                    return (sue1, sue2);
                }
            }
            //return (usize_p(sue), 0);
            //);
        }
        panic!("no such aunt Sue found!");
    }
}
