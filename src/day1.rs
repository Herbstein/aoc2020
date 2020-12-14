use std::collections::HashSet;

#[aoc_generator(day1)]
fn day1_generator(s: &str) -> HashSet<u32> {
    s.lines()
        .filter(|&l| !l.trim().is_empty())
        .filter_map(|l| l.parse().ok())
        .collect()
}

fn pair_sum_to(set: &HashSet<u32>, target: u32) -> Option<(u32, u32)> {
    for l in set {
        let want = target - l;
        if let Some(r) = set.get(&want) {
            return Some((*l, *r));
        }
    }
    None
}

fn triplet_sum_to(set: &HashSet<u32>, target: u32) -> Option<(u32, u32, u32)> {
    for l in set.clone().iter() {
        let new_target = target - l;
        if let Some((m, r)) = pair_sum_to(set, new_target) {
            return Some((*l, m, r));
        }
    }
    None
}

#[aoc(day1, part1)]
pub fn first(nums: &HashSet<u32>) -> u32 {
    let pair = pair_sum_to(nums, 2020);
    let result = pair.map(|(l, r)| l * r);
    result.unwrap()
}

#[aoc(day1, part2)]
pub fn second(nums: &HashSet<u32>) -> u32 {
    let pair = triplet_sum_to(nums, 2020);
    let result = pair.map(|(l, m, r)| l * m * r);
    result.unwrap()
}
