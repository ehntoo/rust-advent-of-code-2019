use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn decode_arg(mode: i64, pc: usize, state: &IntCodeComputerState) -> i64 {
    match mode {
        0 => state.prog[state.prog[pc] as usize],
        1 => state.prog[pc],
        2 => state.prog[(state.prog[pc] + state.rel_base) as usize],
        _ => -1
    }
}

pub fn decode_dst(mode: i64, pc: usize, state: &IntCodeComputerState) -> usize {
    match mode {
        0 => state.prog[pc] as usize,
        1 => pc,
        2 => (state.prog[pc] + state.rel_base) as usize,
        _ => 0
    }
}

pub struct IntCodeComputerState {
    prog: Vec<i64>,
    pc: usize,
    halted: bool,
    rel_base: i64,
}

#[derive(FromPrimitive, Debug)]
enum Opcode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    BranchTrue = 5,
    BranchFalse = 6,
    CmpLessThan = 7,
    CmpEqual = 8,
    SetRel = 9,
    Halt = 99,
}

pub fn run_program(state: &mut IntCodeComputerState, input: i64) -> i64 {
    let mut output = 0;
    while !state.halted {
        let instruction = state.prog[state.pc];
        let opcode = instruction % 100;
        let arg1_mode = || ((instruction / 100) % 10);
        let arg2_mode = || ((instruction / 1000) % 10);
        let arg3_mode = || ((instruction / 10000) % 10);
        let arg1 = || decode_arg(arg1_mode(), state.pc+1, &state);
        let arg2 = || decode_arg(arg2_mode(), state.pc+2, &state);
        let arg3 = || decode_arg(arg3_mode(), state.pc+3, &state);
        // println!("Executing pc {}, instruction {}, opcode {:?}, mode1 {}, mode2 {}, mode3 {}, raw_arg1 {}, raw_arg2 {}, raw_arg3 {}, arg1 {}, arg2 {}, arg3 {}, rel {}",
        //     state.pc, instruction, Opcode::from_i64(opcode).unwrap(), arg1_mode(), arg2_mode(), arg3_mode(),
        //     state.prog[state.pc+1], state.prog[state.pc+2], state.prog[state.pc+3], 
        //     arg1(), arg2(), arg3(), state.rel_base);
        match Opcode::from_i64(opcode) {
            Some(Opcode::Add) => {
                let dst = decode_dst(arg3_mode(), state.pc+3, &state);
                state.prog[dst] = arg1() + arg2();
                state.pc += 4
            },
            Some(Opcode::Multiply) => {
                let dst = decode_dst(arg3_mode(), state.pc+3, &state);
                state.prog[dst] = arg1() * arg2();
                state.pc += 4
            },
            Some(Opcode::Input) => {
                let dst = decode_dst(arg1_mode(), state.pc+1, &state);
                println!("inputing, storing {} to {}", input, dst);
                state.prog[dst] = input;
                state.pc += 2;
                break;
            },
            Some(Opcode::Output) => {
                println!("outputing from {}", state.pc+1);
                // println!("{:?}", state.prog[..25]);
                output = arg1();
                state.pc += 2;
                break;
            },
            Some(Opcode::BranchTrue) => state.pc = if arg1() != 0 { arg2() as usize} else { state.pc + 3 },
            Some(Opcode::BranchFalse) => state.pc = if arg1() == 0 { arg2() as usize} else { state.pc + 3 },
            Some(Opcode::CmpLessThan) => {
                let dst = decode_dst(arg3_mode(), state.pc+3, &state);
                state.prog[dst] = if arg1() < arg2() {1} else {0};
                state.pc += 4
            },
            Some(Opcode::CmpEqual) => {
                let dst = decode_dst(arg3_mode(), state.pc+3, &state);
                state.prog[dst] = if arg1() == arg2() {1} else {0};
                state.pc += 4
            },
            Some(Opcode::SetRel) => {
                state.rel_base += arg1();
                state.pc += 2;
            },
            _ => {
                state.halted = true
            },
        };
    }
    output
}

#[aoc(day9, part1)]
pub fn part1(program: &[i64]) -> i64 {
    let mut computer = IntCodeComputerState {
        prog: program.to_vec(),
        pc: 0,
        halted: false,
        rel_base: 0,
    };
    computer.prog.extend_from_slice(&vec![0; 1024]);
    while !computer.halted {
        println!("{},", run_program(&mut computer, 1));
    }
    0
}

#[aoc(day9, part2)]
pub fn part2(program: &[i64]) -> i64 {
    let mut computer = IntCodeComputerState {
        prog: program.to_vec(),
        pc: 0,
        halted: false,
        rel_base: 0,
    };
    computer.prog.extend_from_slice(&vec![0; 1024]);
    while !computer.halted {
        println!("{},", run_program(&mut computer, 2));
    }
    0
}
