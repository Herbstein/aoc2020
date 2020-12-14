use std::{convert::TryInto, ops::Range};

#[derive(Debug)]
enum FrontBack {
    Front,
    Back,
}

#[derive(Debug)]
enum LeftRight {
    Left,
    Right,
}

struct Partition {
    frontback: [FrontBack; 7],
    leftright: [LeftRight; 3],
}

struct Context {
    rows: Range<u8>,
    columns: Range<u8>,
}

pub struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    fn id(&self) -> usize {
        self.row as usize * 8 + self.column as usize
    }
}

fn parse_partition(p: &str) -> Partition {
    Partition {
        frontback: p[..7]
            .chars()
            .map(|c| match c {
                'F' => FrontBack::Front,
                'B' => FrontBack::Back,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        leftright: p[7..10]
            .chars()
            .map(|c| match c {
                'L' => LeftRight::Left,
                'R' => LeftRight::Right,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

fn partition_naive(p: &Partition) -> Seat {
    let mut context = Context {
        rows: 0..128,
        columns: 0..8,
    };

    for frontback in p.frontback.iter() {
        let distance = (context.rows.end - context.rows.start) / 2;
        match frontback {
            FrontBack::Front => context.rows = context.rows.start..(context.rows.end - distance),
            FrontBack::Back => context.rows = (context.rows.start + distance)..context.rows.end,
        }
    }

    for leftright in p.leftright.iter() {
        let distance = (context.columns.end - context.columns.start) / 2;
        match leftright {
            LeftRight::Left => {
                context.columns = context.columns.start..(context.columns.end - distance)
            }
            LeftRight::Right => {
                context.columns = (context.columns.start + distance)..context.columns.end
            }
        }
    }

    Seat {
        row: context.rows.start,
        column: context.columns.start,
    }
}

/*
#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(parse_partition)
        .map(|p| partition_naive(&p))
        .map(|s| s.id())
        .collect()
}
*/

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

#[test]
fn test() {
    let p = parse_partition("BFFFBBFRRR");
    let seat = partition_naive(&p);
    println!("{}", seat.id());
}
