use crate::solutions::*;

pub struct Solver;

fn is_nice(string: &str) -> bool {
    let mut vowels = 0;
    let mut repeat = false;
    if let Some(a) = string.as_bytes().first() {
        if "aeiou".contains(*a as char) {
            vowels += 1;
        }
    }
    for window in string.as_bytes().windows(2) {
        match (window[0], window[1]) {
            (b'a', b'b') | (b'p', b'q') | (b'c', b'd') | (b'x', b'y') => return false,
            (_, b) => {
                if "aeiou".contains(b as char) {
                    vowels += 1;
                }
            }
        }
        if window[0] == window[1] {
            repeat = true;
        }
    }
    repeat && vowels >= 3
}

fn is_even_nicer(string: &str) -> bool {
    let mut pairs = HashMap::<[&u8; 2], Vec<usize>>::new();
    let mut pair = false;
    let mut repeat = false;
    let mut i = 0;
    let bytes = string.as_bytes();
    for window in bytes.windows(2) {
        if let [a, b] = window {
            if !pair {
                if let Some(entry) = pairs.get_mut(&[a, b]) {
                    if entry.iter().any(|&x| x != i) {
                        pair = true;
                    }
                    entry.push(i + 1);
                } else {
                    pairs.insert([a, b], vec![i + 1]);
                }
            }
            if i > 1 && *b == bytes[i - 1] {
                repeat = true;
            }
            i += 1;
        }
    }
    pair && repeat
}

#[test]
fn nice() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("aaab"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));

    assert!(is_even_nicer("qjhvhtzxzqqjkmpb"));
    assert!(is_even_nicer("xxyxx"));
    assert!(is_even_nicer("aaaa"));
    assert!(is_even_nicer("aaaaa"));
    assert!(is_even_nicer("aaiaaa"));
    assert!(!is_even_nicer("aaa"));
    assert!(!is_even_nicer("uurcxstgmygtbstg"));
    assert!(!is_even_nicer("ieodomkazucvgmuy"));
}

impl Solution for Solver {
    type Answer = usize;
    fn solve(input: Input) -> (Self::Answer, Self::Answer) {
        let mut nice = HashSet::new();
        let mut even_nicer = HashSet::new();
        for string in input {
            if is_nice(string) {
                nice.insert(string);
            }
            if is_even_nicer(string) {
                even_nicer.insert(string);
            }
        }
        (nice.len(), even_nicer.len())
    }
}
