use std::cmp::Ordering;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn is_valid(window: &[usize], target: usize) -> bool {
    for l in window {
        let want = target - l;
        if window.contains(&want) {
            return true;
        }
    }
    false
}

#[aoc(day9, part1)]
fn first(input: &[usize]) -> usize {
    input
        .windows(26)
        .filter_map(|w| {
            let window = &w[..25];
            let target = w[25];
            if !is_valid(window, target) {
                Some(target)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[aoc(day9, part2)]
fn second(input: &[usize]) -> usize {
    let target = first(input);

    let (mut i, mut j) = (0, 0);
    let mut sum = input[0];
    loop {
        match sum.cmp(&target) {
            Ordering::Less => {
                j += 1;
                sum += input[j];
            }
            Ordering::Equal => {
                let range = &input[i..=j];
                break range.iter().min().unwrap() + range.iter().max().unwrap();
            }
            Ordering::Greater => {
                sum -= input[i];
                i += 1;
            }
        }
    }
}
