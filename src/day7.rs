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

pub struct IntCodeComputerState {
    prog: Vec<i32>,
    pc: usize,
    halted: bool,
}

pub fn run_program(state: &mut IntCodeComputerState, input: i32) -> i32 {
    let mut output = 0;
    while !state.halted {
        let opcode = state.prog[state.pc] % 100;
        let arg1_mode = (state.prog[state.pc] / 100) % 10;
        let arg2_mode = (state.prog[state.pc] / 1000) % 10;
        let arg1 = || decode_arg(arg1_mode, state.pc+1, &state.prog);
        let arg2 = || decode_arg(arg2_mode, state.pc+2, &state.prog);
        match opcode {
            1 => {
                let dst = state.prog[state.pc+3] as usize;
                state.prog[dst] = arg1() + arg2();
                state.pc += 4
            },
            2 => {
                let dst = state.prog[state.pc+3] as usize;
                state.prog[dst] = arg1() * arg2();
                state.pc += 4
            },
            3 => {
                let dst = state.prog[state.pc+1] as usize;
                state.prog[dst] = input;
                state.pc += 2;
                break;
            },
            4 => {
                output = arg1();
                state.pc += 2;
                break;
            },
            5 => state.pc = if arg1() != 0 { arg2() as usize} else { state.pc + 3 },
            6 => state.pc = if arg1() == 0 { arg2() as usize} else { state.pc + 3 },
            7 => {
                let dst = state.prog[state.pc+3] as usize;
                state.prog[dst] = if arg1() < arg2() {1} else {0};
                state.pc += 4
            },
            8 => {
                let dst = state.prog[state.pc+3] as usize;
                state.prog[dst] = if arg1() == arg2() {1} else {0};
                state.pc += 4
            },
            _ => {
                state.halted = true
            },
        };
    }
    output
}

#[aoc(day7, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut max_output = 0;
    for perm in (0..5).permutations(5) {
        let mut amplifier_computers: Vec<IntCodeComputerState> = (0..5).map(|_| {
            IntCodeComputerState {
                prog: program.to_vec(),
                pc: 0,
                halted: false,
            }
        }).collect();
        run_program(&mut amplifier_computers[0], perm[0]);
        run_program(&mut amplifier_computers[0], 0);
        let out = run_program(&mut amplifier_computers[0], 0);
        run_program(&mut amplifier_computers[1], perm[1]);
        run_program(&mut amplifier_computers[1], out);
        let out = run_program(&mut amplifier_computers[1], out);
        run_program(&mut amplifier_computers[2], perm[2]);
        run_program(&mut amplifier_computers[2], out);
        let out = run_program(&mut amplifier_computers[2], out);
        run_program(&mut amplifier_computers[3], perm[3]);
        run_program(&mut amplifier_computers[3], out);
        let out = run_program(&mut amplifier_computers[3], out);
        run_program(&mut amplifier_computers[4], perm[4]);
        run_program(&mut amplifier_computers[4], out);
        let out = run_program(&mut amplifier_computers[4], out);
        if out > max_output {
            max_output = out;
        }
    }
    max_output
}

#[aoc(day7, part2)]
pub fn part2(_program: &[i32]) -> i32 {
    // let mut max_output = 0;
    // for perm in (5..10).permutations(5) {
    //     let mut amplifier_computers: Vec<IntCodeComputerState> = (0..5).map(|_| {
    //         IntCodeComputerState {
    //             prog: program.to_vec(),
    //             pc: 0,
    //         }
    //     }).collect();
    //     let out = run_program(&mut amplifier_computers[0], &vec![perm[0], 0]);
    //     let out = run_program(&mut amplifier_computers[1], &vec![perm[1], out[0]]);
    //     let out = run_program(&mut amplifier_computers[2], &vec![perm[2], out[0]]);
    //     let out = run_program(&mut amplifier_computers[3], &vec![perm[3], out[0]]);
    //     let out = run_program(&mut amplifier_computers[4], &vec![perm[4], out[0]]);
    //     if out[0] > max_output {
    //         max_output = out[0];
    //     }
    // }
    // max_output
    0
}
