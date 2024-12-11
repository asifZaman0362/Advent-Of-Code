use crate::solutions::*;
use std::collections::{HashMap, HashSet};

//pub struct Solver;

fn solve0(input: &crate::solutions::Input) -> HashMap<String, u32> {
    let mut stack = vec!["root"];
    let mut files: HashMap<String, HashSet<String>> = HashMap::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    sizes.insert(String::from("root"), 0);
    files.insert(String::from("root"), HashSet::new());
    let mut cwd = String::from("root");
    for line in input.split('\n') {
        if line.starts_with("$") {
            let mut iter = line[2..].split(" ");
            if let Some("cd") = iter.next() {
                if let Some(dir) = iter.next() {
                    if dir == ".." {
                        stack.pop();
                    } else if dir == "/" {
                        stack.clear();
                        stack.push("root");
                        cwd = String::from("root");
                    } else {
                        stack.push(dir);
                    }
                }
            } else {
                // cmd is ls
                cwd = stack.join("/");
                if !sizes.contains_key(&cwd) {
                    sizes.insert(cwd.clone(), 0);
                }
                if !files.contains_key(&cwd) {
                    files.insert(cwd.clone(), HashSet::new());
                }
            }
        } else {
            // line is a file / directory in cwd
            let (size, name) = line.split_once(" ").unwrap();
            if let Ok(size) = size.parse::<u32>() {
                let mut path = String::from("root");
                if let Some(list) = files.get_mut(&cwd) {
                    if list.get(name).is_none() {
                        *sizes.get_mut("root").unwrap() += size;
                        for sub in &stack[1..] {
                            path.push('/');
                            path.push_str(sub);
                            //dbg!(&path);
                            *sizes.get_mut(&path).unwrap() += size;
                        }
                        list.insert(name.to_string());
                    }
                }
            }
        }
    }
    //dbg!(&sizes);
    //dbg!(&files);
    sizes
}

//fn solve1(input: &crate::solutions::Input) -> u32 {}

/*impl Solution for Solver {
type Answer = u32;
fn solve(input: crate::solutions::Input) -> (Self::Answer, Self::Answer) {*/
r#macro::solution!(2022, 7, u32, {
    let values = solve0(&input);
    let target = 30_000_000 - (70_000_000 - values.get("root").unwrap());
    let first = values.values().filter(|&&x| x <= 100_000).sum();
    let second = values.values().filter(|&&x| x >= target).min().unwrap();
    //(solve0(&input), solve1(&input))
    (first, *second)
});
