use crate::solutions::*;

//pub struct Solver;

/*impl Solution for Solver {
type Answer = u64;
fn solve(input: Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2024, 5, u64, {
    let input = input.lines().collect::<Vec<_>>();
    let order_end = input.iter().position(|str| str.is_empty()).unwrap();
    let requirements = input[..order_end]
        .iter()
        .map(|str| {
            str.split_once("|")
                .map(|(before, after)| (u64_p(after), u64_p(before)))
                .unwrap()
        })
        .fold(
            HashMap::<u64, HashSet<u64>>::new(),
            |mut map, (before, after)| {
                map.entry(before)
                    .and_modify(|val| {
                        val.insert(after);
                    })
                    .or_insert(HashSet::from([after]));
                map
            },
        );
    input[order_end + 1..]
        .iter()
        .map(|str| str.split(",").map(u64_p).collect::<Vec<_>>())
        .fold((0, 0), |(part1, part2), mut update| {
            if update.is_sorted_by(|before: &u64, after: &u64| {
                requirements
                    .get(before)
                    .map_or(true, |set| !set.contains(after))
            }) {
                (part1 + update[update.len() / 2], part2)
            } else {
                update.sort_by(|before, after| {
                    requirements.get(before).map_or(Ordering::Less, |set| {
                        set.get(after).map_or(Ordering::Less, |_| Ordering::Greater)
                    })
                });
                (part1, part2 + update[update.len() / 2])
            }
        })
});
