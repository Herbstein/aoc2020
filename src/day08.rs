#[derive(Clone)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let operation = match &l[..3] {
                "nop" => Operation::Nop,
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                _ => unreachable!(),
            };
            let argument = l[4..].parse().unwrap();
            Instruction {
                operation,
                argument,
            }
        })
        .collect()
}

fn run(instructions: &[Instruction]) -> (bool, i64, Vec<bool>) {
    let mut seen = vec![false; instructions.len()];
    let mut pc = 0;
    let mut acc = 0;

    loop {
        let idx = pc as usize;
        if idx == instructions.len() - 1 {
            break (true, acc, seen);
        }

        let Instruction {
            operation,
            argument,
        } = &instructions[idx];
        let s = &mut seen[idx];

        if *s {
            break (false, acc, seen);
        }

        match operation {
            Operation::Acc => {
                acc += argument;
            }
            Operation::Nop => (),
            Operation::Jmp => pc += argument - 1,
        }

        pc += 1;
        *s = true;
    }
}

#[aoc(day8, part1)]
fn first(input: &[Instruction]) -> i64 {
    let (_, acc, _) = run(input);
    acc
}

#[aoc(day8, part2)]
fn second(input: &[Instruction]) -> i64 {
    let mut instructions = input.to_vec();
    let (_, _, trace) = run(&instructions);

    let mut potential_landing_spots = vec![false; instructions.len() + 1];
    let mut i = instructions.len();

    loop {
        potential_landing_spots[i] = true;
        i -= 1;

        if let Instruction {
            operation: Operation::Jmp,
            argument,
        } = instructions[i]
        {
            if argument < 0 {
                break;
            }
        }
    }

    let start = i;
    let swap = if trace[i] {
        i
    } else {
        loop {
            i -= 1;

            if potential_landing_spots[i] {
                continue;
            }

            let instr = &instructions[i];
            match instr.operation {
                Operation::Nop => {
                    if trace[i] && potential_landing_spots[(i as i64 + instr.argument) as usize] {
                        break i;
                    }
                }
                Operation::Jmp => {
                    if !trace[i]
                        && potential_landing_spots[((i as i64 + instr.argument) as usize)]
                            & !potential_landing_spots[i]
                    {
                        let mut j = i - 1;
                        loop {
                            if matches!(instructions[j].operation, Operation::Jmp) {
                                break;
                            }
                            j -= 1;
                        }

                        if trace[j] {
                            break j;
                        } else {
                            potential_landing_spots[j + 1..=i].iter_mut().for_each(|a| {
                                *a = true;
                            });
                            i = start;
                        }
                    }
                }
                _ => (),
            }
        }
    };

    instructions[swap].operation = match instructions[swap].operation {
        Operation::Acc => unreachable!(),
        Operation::Jmp => Operation::Nop,
        Operation::Nop => Operation::Jmp,
    };

    let (term, acc, _) = run(&instructions);
    assert!(term);
    acc
}
