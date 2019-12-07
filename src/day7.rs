use itertools::Itertools;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn decode_arg(mode: i32, pc: usize, intcode: &[i32]) -> i32 {
    match mode {
        0 => intcode[intcode[pc] as usize],
        1 => intcode[pc],
        _ => -1
    }
}

pub fn run_program(intcode: &mut Vec<i32>, input: &[i32]) -> Vec<i32> {
    let mut pc = 0;
    let mut input_idx = 0;
    let mut output = vec![];
    loop {
        let opcode = intcode[pc] % 100;
        let arg1_mode = (intcode[pc] / 100) % 10;
        let arg2_mode = (intcode[pc] / 1000) % 10;
        let arg1 = || decode_arg(arg1_mode, pc+1, intcode);
        let arg2 = || decode_arg(arg2_mode, pc+2, intcode);
        match opcode {
            1 => {
                let dst = intcode[pc+3] as usize;
                intcode[dst] = arg1() + arg2();
                pc += 4
            },
            2 => {
                let dst = intcode[pc+3] as usize;
                intcode[dst] = arg1() * arg2();
                pc += 4
            },
            3 => {
                let dst = intcode[pc+1] as usize;
                intcode[dst] = input[input_idx];
                input_idx += 1;
                pc += 2
            },
            4 => {
                output.push(arg1());
                pc += 2
            },
            5 => pc = if arg1() != 0 { arg2() as usize} else { pc + 3 },
            6 => pc = if arg1() == 0 { arg2() as usize} else { pc + 3 },
            7 => {
                let dst = intcode[pc+3] as usize;
                intcode[dst] = if arg1() < arg2() {1} else {0};
                pc += 4
            },
            8 => {
                let dst = intcode[pc+3] as usize;
                intcode[dst] = if arg1() == arg2() {1} else {0};
                pc += 4
            },
            _ => break,
        };
    }
    output
}

#[aoc(day7, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut max_output = 0;
    for perm in (0..5).permutations(5) {
        let out = run_program(&mut program.to_vec(), &vec![perm[0], 0]);
        let out = run_program(&mut program.to_vec(), &vec![perm[1], out[0]]);
        let out = run_program(&mut program.to_vec(), &vec![perm[2], out[0]]);
        let out = run_program(&mut program.to_vec(), &vec![perm[3], out[0]]);
        let out = run_program(&mut program.to_vec(), &vec![perm[4], out[0]]);
        if out[0] > max_output {
            max_output = out[0];
        }
    }
    max_output
}

#[aoc(day7, part2)]
pub fn part2(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    run_program(&mut local_program, &vec![0, 5]);
    0
}
