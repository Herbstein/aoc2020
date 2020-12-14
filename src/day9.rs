use std::collections::{BTreeSet, HashSet};

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn is_valid_slice(window: &[usize], num: usize) -> bool {
    for pre in window {
        let want = pre - num;
        if window.contains(&want) {
            return true;
        }
    }
    false
}

fn is_valid_hash(window: &[usize], num: usize) -> bool {
    let set = window.iter().cloned().collect::<HashSet<_>>();
    for pre in set.iter() {
        let want = pre - num;
        if set.contains(&want) {
            return true;
        }
    }
    false
}

fn is_valid_tree(window: &[usize], num: usize) -> bool {
    let set = window.iter().cloned().collect::<BTreeSet<_>>();
    for pre in set.iter() {
        let want = pre - num;
        if set.contains(&want) {
            return true;
        }
    }
    false
}

#[aoc(day9, part1, slice)]
fn first(input: &[usize]) -> usize {
    input
        .windows(26)
        .filter_map(|w| {
            let preamble = &w[..25];
            let num = w[25];
            if is_valid_slice(preamble, num) {
                Some(num)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[aoc(day9, part1, hash)]
fn first_hash(input: &[usize]) -> usize {
    input
        .windows(26)
        .filter_map(|w| {
            let preamble = &w[..25];
            let num = w[25];
            if is_valid_hash(preamble, num) {
                Some(num)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[aoc(day9, part1, tree)]
fn first_tree(input: &[usize]) -> usize {
    input
        .windows(26)
        .filter_map(|w| {
            let preamble = &w[..25];
            let num = w[25];
            if is_valid_tree(preamble, num) {
                Some(num)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
