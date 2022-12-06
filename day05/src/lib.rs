use itertools::Itertools;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn program_a(input: &str) {
    let (left, instruction_str) = input.split_once("\n\n").unwrap();
    let (stack_str, platforms) = left.rsplit_once("\n").unwrap();
    let num_stacks = platforms.split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks = vec![Vec::new(); num_stacks];

    for line in stack_str.lines().rev() {
        for (idx, mut chunk) in line.chars().chunks(4).into_iter().enumerate(){
            let second = chunk.nth(1).unwrap();
            if second.is_alphabetic() {
                stacks[idx].push(second)
            }
        }
    }

    let mut instructions = Vec::new();
    for line in instruction_str.lines() {
        let rest = line.strip_prefix("move ").unwrap();
        let (amount, rest) = rest.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let instruction = Instruction {
            amount: amount.parse().ok().unwrap(),
            from: from.parse::<usize>().ok().unwrap() - 1,
            to: to.parse::<usize>().ok().unwrap() - 1,
        };
        instructions.push(instruction);
    }

    for Instruction { amount, from, to } in instructions {
        let from_stack_len = stacks[from].len();
        let removed = stacks[from].split_off(from_stack_len - amount);
        stacks[to].extend(removed);
    }

    println!("{}",stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect::<String>())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_program_a() {
        program_a("    [P]                 [C] [C]
    [W]         [B]     [G] [V] [V]
    [V]         [T] [Z] [J] [T] [S]
    [D] [L]     [Q] [F] [Z] [W] [R]
    [C] [N] [R] [H] [L] [Q] [F] [G]
[F] [M] [Z] [H] [G] [W] [L] [R] [H]
[R] [H] [M] [C] [P] [C] [V] [N] [W]
[W] [T] [P] [J] [C] [G] [W] [P] [J]
 1   2   3   4   5   6   7   8   9

move 2 from 4 to 9
move 5 from 2 to 9
move 1 from 5 to 1
move 3 from 1 to 4
move 2 from 4 to 6
move 7 from 6 to 9
move 5 from 3 to 9
move 1 from 8 to 6
move 3 from 2 to 3
move 12 from 9 to 3
move 4 from 9 to 7
move 15 from 3 to 9
move 1 from 1 to 5
move 2 from 6 to 5
move 18 from 9 to 1
move 6 from 8 to 1
move 1 from 8 to 7
move 5 from 7 to 2
move 6 from 1 to 2
move 7 from 9 to 6
move 6 from 1 to 3
move 5 from 3 to 9
move 3 from 9 to 1
move 1 from 7 to 9
move 4 from 2 to 1
move 5 from 6 to 3
move 1 from 3 to 4
move 1 from 5 to 9
move 2 from 9 to 6
move 5 from 5 to 9
move 10 from 1 to 8
move 4 from 3 to 8
move 3 from 4 to 9
move 4 from 6 to 9
move 14 from 8 to 6
move 1 from 3 to 8
move 14 from 9 to 4
move 6 from 1 to 6
move 1 from 8 to 2
move 3 from 5 to 8
move 1 from 8 to 9
move 1 from 8 to 1
move 5 from 4 to 9
move 1 from 8 to 4
move 3 from 9 to 4
move 3 from 7 to 5
move 7 from 6 to 3
move 7 from 4 to 1
move 3 from 9 to 1
move 7 from 2 to 3
move 1 from 4 to 8
move 8 from 6 to 2
move 2 from 7 to 4
move 1 from 7 to 4
move 1 from 7 to 9
move 1 from 5 to 9
move 1 from 9 to 4
move 1 from 4 to 2
move 8 from 4 to 9
move 1 from 4 to 2
move 5 from 9 to 4
move 2 from 6 to 9
move 1 from 6 to 9
move 1 from 8 to 1
move 13 from 3 to 2
move 1 from 3 to 9
move 2 from 6 to 8
move 1 from 8 to 1
move 14 from 1 to 7
move 4 from 2 to 1
move 2 from 9 to 5
move 3 from 9 to 7
move 1 from 8 to 2
move 4 from 1 to 5
move 1 from 4 to 7
move 3 from 9 to 1
move 7 from 7 to 4
move 14 from 2 to 8
move 3 from 1 to 7
move 3 from 5 to 4
move 2 from 1 to 9
move 11 from 8 to 9
move 3 from 7 to 8
move 3 from 8 to 6
move 6 from 4 to 3
move 2 from 6 to 8
move 8 from 4 to 3
move 3 from 8 to 7
move 2 from 8 to 2
move 2 from 3 to 9
move 1 from 6 to 8
move 5 from 2 to 7
move 10 from 9 to 7
move 1 from 8 to 5
move 3 from 5 to 2
move 6 from 7 to 5
move 19 from 7 to 3
move 9 from 5 to 9
move 6 from 2 to 6
move 2 from 7 to 3
move 29 from 3 to 8
move 2 from 7 to 9
move 5 from 8 to 1
move 12 from 9 to 6
move 1 from 3 to 8
move 1 from 2 to 7
move 1 from 3 to 1
move 10 from 6 to 1
move 1 from 6 to 7
move 9 from 1 to 9
move 2 from 1 to 2
move 12 from 9 to 4
move 7 from 6 to 3
move 8 from 3 to 7
move 5 from 7 to 6
move 19 from 8 to 3
move 10 from 4 to 6
move 1 from 4 to 6
move 6 from 8 to 6
move 1 from 4 to 2
move 6 from 6 to 3
move 3 from 2 to 7
move 13 from 6 to 3
move 1 from 9 to 1
move 6 from 1 to 8
move 1 from 6 to 5
move 1 from 5 to 4
move 3 from 7 to 1
move 2 from 1 to 3
move 11 from 3 to 8
move 1 from 4 to 3
move 3 from 8 to 4
move 1 from 7 to 5
move 3 from 8 to 9
move 2 from 9 to 2
move 7 from 8 to 3
move 1 from 7 to 9
move 1 from 1 to 4
move 32 from 3 to 4
move 1 from 5 to 9
move 2 from 8 to 3
move 2 from 6 to 4
move 1 from 9 to 4
move 1 from 9 to 2
move 3 from 3 to 1
move 1 from 8 to 6
move 1 from 6 to 2
move 1 from 9 to 3
move 1 from 1 to 7
move 1 from 8 to 7
move 2 from 3 to 8
move 1 from 8 to 4
move 1 from 1 to 2
move 2 from 4 to 8
move 1 from 1 to 8
move 26 from 4 to 6
move 3 from 8 to 5
move 3 from 7 to 6
move 7 from 6 to 3
move 18 from 6 to 8
move 16 from 8 to 9
move 1 from 5 to 1
move 2 from 8 to 3
move 3 from 9 to 8
move 3 from 6 to 4
move 2 from 5 to 4
move 1 from 6 to 4
move 2 from 7 to 2
move 2 from 3 to 9
move 4 from 8 to 3
move 1 from 1 to 2
move 6 from 9 to 7
move 2 from 2 to 5
move 12 from 3 to 1
move 9 from 9 to 2
move 10 from 1 to 3
move 2 from 5 to 9
move 8 from 4 to 7
move 13 from 7 to 6
move 6 from 6 to 5
move 4 from 5 to 3
move 2 from 5 to 4
move 8 from 4 to 3
move 1 from 7 to 2
move 15 from 2 to 7
move 8 from 3 to 7
move 1 from 1 to 6
move 7 from 7 to 1
move 5 from 1 to 6
move 7 from 3 to 2
move 3 from 1 to 6
move 12 from 7 to 9
move 12 from 9 to 8
move 1 from 7 to 1
move 2 from 9 to 5
move 1 from 1 to 9
move 4 from 4 to 2
move 4 from 8 to 4
move 2 from 7 to 2
move 4 from 6 to 5
move 4 from 8 to 9
move 1 from 8 to 4
move 5 from 5 to 3
move 5 from 2 to 4
move 5 from 9 to 5
move 1 from 3 to 6
move 1 from 7 to 8
move 12 from 3 to 9
move 4 from 2 to 6
move 7 from 4 to 9
move 13 from 6 to 4
move 3 from 6 to 9
move 4 from 4 to 2
move 1 from 3 to 4
move 21 from 9 to 7
move 4 from 2 to 1
move 3 from 5 to 4
move 8 from 7 to 6
move 2 from 7 to 2
move 11 from 4 to 2
move 1 from 9 to 7
move 1 from 5 to 7
move 1 from 1 to 8
move 5 from 2 to 5
move 1 from 3 to 5
move 2 from 4 to 9
move 3 from 4 to 8
move 3 from 1 to 8
move 1 from 9 to 6
move 8 from 7 to 8
move 9 from 6 to 5
move 1 from 9 to 6
move 1 from 6 to 4
move 3 from 7 to 5
move 1 from 6 to 9
move 12 from 5 to 1
move 2 from 5 to 8
move 1 from 9 to 6
move 2 from 7 to 6
move 9 from 1 to 8
move 1 from 6 to 9
move 1 from 9 to 2
move 1 from 4 to 2
move 2 from 6 to 7
move 5 from 8 to 3
move 2 from 7 to 4
move 16 from 8 to 5
move 2 from 3 to 8
move 7 from 5 to 1
move 3 from 3 to 8
move 7 from 5 to 7
move 4 from 5 to 2
move 6 from 7 to 9
move 2 from 9 to 6
move 2 from 9 to 2
move 1 from 6 to 8
move 12 from 2 to 6
move 2 from 9 to 6
move 1 from 5 to 2
move 3 from 5 to 4
move 9 from 2 to 6
move 6 from 8 to 3
move 1 from 7 to 5
move 1 from 6 to 7
move 1 from 7 to 8
move 1 from 5 to 8
move 5 from 1 to 2
move 3 from 4 to 5
move 4 from 6 to 8
move 5 from 2 to 9
move 5 from 8 to 4
move 1 from 1 to 4
move 9 from 8 to 4
move 1 from 2 to 3
move 3 from 6 to 8
move 4 from 9 to 2
move 2 from 6 to 4
move 2 from 3 to 1
move 4 from 4 to 7
move 6 from 4 to 5
move 10 from 6 to 8
move 4 from 1 to 9
move 4 from 7 to 5
move 3 from 3 to 9
move 6 from 9 to 8
move 2 from 2 to 9
move 8 from 4 to 3
move 2 from 2 to 7
move 1 from 4 to 9
move 6 from 3 to 8
move 2 from 7 to 8
move 6 from 5 to 9
move 5 from 5 to 6
move 2 from 5 to 9
move 7 from 9 to 5
move 2 from 1 to 9
move 6 from 5 to 8
move 1 from 5 to 1
move 2 from 3 to 6
move 1 from 3 to 6
move 4 from 9 to 5
move 1 from 3 to 4
move 1 from 1 to 2
move 1 from 2 to 1
move 1 from 6 to 8
move 14 from 8 to 5
move 6 from 5 to 1
move 16 from 8 to 3
move 2 from 8 to 2
move 10 from 6 to 7
move 1 from 6 to 9
move 2 from 2 to 9
move 2 from 7 to 3
move 1 from 8 to 5
move 3 from 9 to 1
move 4 from 9 to 5
move 9 from 3 to 8
move 2 from 3 to 6
move 5 from 3 to 8
move 1 from 4 to 2
move 12 from 8 to 4
move 1 from 8 to 9
move 4 from 5 to 9
move 7 from 7 to 1
move 10 from 5 to 2
move 2 from 5 to 2
move 1 from 6 to 5
move 2 from 5 to 2
move 5 from 2 to 6
move 4 from 9 to 6
move 6 from 4 to 9
move 2 from 3 to 4
move 6 from 4 to 7
move 6 from 7 to 5
move 10 from 1 to 5
move 4 from 1 to 2
move 4 from 6 to 3
move 6 from 9 to 7
move 2 from 4 to 9
move 7 from 7 to 6
move 1 from 9 to 7
move 2 from 9 to 8
move 2 from 8 to 2
move 1 from 2 to 5
move 3 from 8 to 4
move 4 from 2 to 7
move 3 from 4 to 7
move 2 from 3 to 5
move 2 from 3 to 2
move 18 from 5 to 3
move 6 from 3 to 1
move 8 from 3 to 1
move 8 from 7 to 9
move 9 from 2 to 5
move 3 from 2 to 3
move 7 from 3 to 7
move 3 from 6 to 4
move 1 from 7 to 1
move 7 from 6 to 7
move 1 from 2 to 9
move 1 from 4 to 2
move 13 from 7 to 2
move 10 from 5 to 3
move 1 from 2 to 9
move 7 from 1 to 5
move 8 from 9 to 5
move 1 from 9 to 5
move 1 from 9 to 8
move 1 from 8 to 2
move 8 from 5 to 3
move 18 from 3 to 5
move 2 from 4 to 1
move 3 from 2 to 5
move 27 from 5 to 1
move 17 from 1 to 5
move 2 from 2 to 3
move 1 from 6 to 5
move 2 from 2 to 5
move 1 from 6 to 4
move 1 from 6 to 9
move 2 from 3 to 5
move 17 from 5 to 6
move 1 from 9 to 3
move 6 from 2 to 4
move 1 from 3 to 2
move 3 from 4 to 9
move 1 from 2 to 9
move 1 from 4 to 7
move 3 from 5 to 2
move 2 from 5 to 1
move 1 from 5 to 2
move 1 from 7 to 3
move 18 from 1 to 4
move 1 from 3 to 1
move 5 from 4 to 2
move 1 from 5 to 1
move 9 from 2 to 7
move 1 from 4 to 5
move 1 from 2 to 9
move 8 from 6 to 2
move 13 from 4 to 2
move 2 from 4 to 9
move 1 from 5 to 2
move 1 from 6 to 8
move 6 from 7 to 5
move 1 from 8 to 4
move 1 from 7 to 6
move 1 from 6 to 1
move 7 from 6 to 5
move 1 from 7 to 9
move 6 from 9 to 3
move 2 from 9 to 7
move 2 from 5 to 7
move 4 from 7 to 8
move 4 from 5 to 4
move 1 from 6 to 7
move 3 from 3 to 8
move 6 from 5 to 9
move 2 from 3 to 5
move 4 from 4 to 7
move 1 from 3 to 1
move 2 from 2 to 3
move 6 from 9 to 6
move 1 from 7 to 1
move 19 from 2 to 4
move 2 from 5 to 6
move 2 from 8 to 9
move 2 from 1 to 2
move 2 from 2 to 5
move 2 from 4 to 3
move 4 from 6 to 2
move 1 from 7 to 8
move 6 from 1 to 8
move 3 from 5 to 1
move 5 from 2 to 5
move 1 from 6 to 7
move 9 from 8 to 1
move 2 from 3 to 6
move 4 from 6 to 5
move 1 from 6 to 2
move 9 from 5 to 2
move 3 from 4 to 6
move 12 from 4 to 6
move 1 from 9 to 4
move 1 from 3 to 1
move 3 from 4 to 8
move 1 from 3 to 6
move 6 from 6 to 2
move 1 from 4 to 5
move 3 from 6 to 2
move 4 from 1 to 5
move 1 from 5 to 1
move 2 from 8 to 9
move 7 from 6 to 3
move 1 from 3 to 1
move 1 from 8 to 1
move 3 from 8 to 9
move 4 from 3 to 5
move 3 from 7 to 3
move 5 from 3 to 7
move 1 from 9 to 1
move 4 from 9 to 2
move 15 from 2 to 7
move 14 from 1 to 7
move 5 from 5 to 1
move 9 from 7 to 2
move 1 from 9 to 6
move 1 from 7 to 4
move 1 from 4 to 6
move 2 from 6 to 2
move 9 from 2 to 5
move 4 from 2 to 4
move 4 from 7 to 5
move 6 from 5 to 9
move 7 from 1 to 8
move 6 from 2 to 8
move 1 from 1 to 2
move 3 from 9 to 5
move 18 from 7 to 8
move 2 from 4 to 6
move 2 from 4 to 6
move 3 from 7 to 6
move 3 from 5 to 3
move 1 from 2 to 6
move 5 from 6 to 8
move 29 from 8 to 1
move 2 from 3 to 5
move 25 from 1 to 6
move 2 from 9 to 5
move 1 from 7 to 8
move 6 from 8 to 2
move 1 from 9 to 1
move 15 from 6 to 8
move 1 from 3 to 8
move 14 from 8 to 7
move 5 from 1 to 3
move 1 from 6 to 2
move 2 from 5 to 7
move 10 from 6 to 2
move 4 from 5 to 7
move 6 from 5 to 1
move 2 from 1 to 4
move 19 from 7 to 9")
    }
}
