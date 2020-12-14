#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}
