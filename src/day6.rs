use std::collections::BTreeSet;

type Group = Vec<BTreeSet<char>>;

fn parse_inclusive_group(s: &str) -> Group {
    s.lines().map(|l| l.trim().chars().collect()).collect()
}

fn parse_inclusive_groups(s: &str) -> impl Iterator<Item = Group> + '_ {
    s.split("\n\n").map(|s| parse_inclusive_group(s.trim()))
}

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<Group> {
    parse_inclusive_groups(input.trim()).collect()
}

#[aoc(day6, part1)]
pub fn first(input: &[Group]) -> usize {
    input
        .iter()
        .map(|g| {
            g.iter()
                .fold(BTreeSet::default(), |h, p| h.union(p).cloned().collect())
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn second(input: &[Group]) -> usize {
    input
        .iter()
        .map(|g| {
            let mut iter = g.iter();
            let first = iter.next().unwrap().clone();
            iter.fold(first, |h, p| h.intersection(&p).cloned().collect())
                .len()
        })
        .sum()
}
