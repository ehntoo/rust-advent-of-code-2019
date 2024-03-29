#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn write_int(int: i32) {
    println!("{}", int);
}

pub fn decode_arg(mode: i32, pc: usize, intcode: &[i32]) -> i32 {
    match mode {
        0 => intcode[intcode[pc] as usize],
        1 => intcode[pc],
        _ => -1
    }
}

pub fn run_program(intcode: &mut Vec<i32>, input: i32) -> i32 {
    let mut pc = 0;
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
                intcode[dst] = input;
                pc += 2
            },
            4 => {
                write_int(arg1());
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
    intcode[0]
}

#[aoc(day5, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    run_program(&mut local_program, 1)
}

#[aoc(day5, part2)]
pub fn part2(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    run_program(&mut local_program, 5)
}
