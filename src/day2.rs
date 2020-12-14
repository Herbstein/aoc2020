use std::convert::TryInto;

struct Policy {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Policy {
    fn is_valid_range(&self) -> bool {
        let number = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count();
        number >= self.min && self.max >= number
    }

    fn is_valid_index(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        (chars[self.min - 1] == self.character && chars[self.max - 1] != self.character)
            || (chars[self.min - 1] != self.character && chars[self.max - 1] == self.character)
    }

    fn is_valid_index_alternate(&self) -> bool {
        self.password
            .char_indices()
            .filter(|(idx, ch)| (*idx == self.min || *idx == self.max) && *ch == self.character)
            .count()
            == 1
    }
}

fn parse_policy_line(line: &str) -> Policy {
    let [policy, pass]: [&str; 2] = line
        .trim()
        .split(':')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let [range, char_str]: [&str; 2] = policy
        .trim()
        .split(' ')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let [min, max]: [&str; 2] = range
        .trim()
        .split('-')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let policy = Policy {
        min: min.trim().parse().unwrap(),
        max: max.trim().parse().unwrap(),
        character: char_str.chars().next().unwrap(),
        password: pass.trim().to_string(),
    };
    policy
}

#[aoc_generator(day2)]
fn day2_generator(input: &str) -> Vec<Policy> {
    input.lines().map(parse_policy_line).collect()
}

#[aoc(day2, part1)]
fn part1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| policy.is_valid_range())
        .count()
}

#[aoc(day2, part2)]
fn part2(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| policy.is_valid_index())
        .count()
}

#[aoc(day2, part2, full_iter)]
fn part2_alternate(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| policy.is_valid_index_alternate())
        .count()
}

#[aoc(day2, part2, for_loop)]
fn part2_for(policies: &[Policy]) -> usize {
    let mut count = 0;
    for policy in policies {
        if policy.is_valid_index_alternate() {
            count += 1;
        }
    }
    count
}
