#[aoc_generator(day5)]
fn partition_to_id_smart(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            l.replace('B', "1")
                .replace('R', "1")
                .replace('F', "0")
                .replace('L', "0")
        })
        .map(|l| usize::from_str_radix(&l, 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn first(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn second(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable();

    input
        .windows(2)
        .filter_map(|w| {
            if w[0] + 1 != w[1] {
                Some(w[0] + 1)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
