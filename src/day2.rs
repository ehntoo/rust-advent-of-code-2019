#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    local_program[1] = 12;
    local_program[2] = 2;
    run_program(&mut local_program)
}

#[aoc(day2, part2)]
pub fn part2(program: &[i32]) -> i32 {
    let target_output = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut local_program = program.to_vec();
            local_program[1] = noun;
            local_program[2] = verb;
            let result = run_program(&mut local_program);
            if result == target_output {
                return 100 * noun + verb;
            }
        }
    }
    -1
}

pub fn run_program(intcode: &mut Vec<i32>) -> i32 {
    let mut pc = 0;
    loop {
        let opcode = intcode[pc];
        let src_1 = intcode[pc + 1] as usize;
        let src_2 = intcode[pc + 2] as usize;
        let dst = intcode[pc + 3] as usize;
        match opcode {
            1 => intcode[dst] = intcode[src_1] + intcode[src_2],
            2 => intcode[dst] = intcode[src_1] * intcode[src_2],
            _ => break,
        };
        pc += 4;
    }
    intcode[0]
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn sample1() {
        assert_eq!(run_program(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50]), 3500);
    }
}
