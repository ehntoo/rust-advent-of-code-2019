#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

pub fn read_int() -> i32 {
    return 1;
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

pub fn run_program(intcode: &mut Vec<i32>) -> i32 {
    let mut pc = 0;
    loop {
        // println!("Processing opcode {}, arg1 {} arg2 {} arg3 {}", intcode[pc], intcode[pc+1], intcode[pc+2], intcode[pc+3]);
        let opcode = intcode[pc] % 100;
        let arg1_mode = (intcode[pc] / 100) % 10;
        let arg2_mode = (intcode[pc] / 1000) % 10;
        // let arg3_mode = intcode[pc] / 10000;
        let raw_arg_1 = intcode[pc+1];
        let raw_arg_3 = intcode[pc+3];
        // println!("arg1_mode {} arg2_mode {} arg3_mode {}", arg1_mode, arg2_mode, arg3_mode);
        // println!("{}", intcode[225]);
        // println!("{:?}", intcode);
        let decode_arg1 = || decode_arg(arg1_mode, pc+1, intcode);
        let decode_arg2 = || decode_arg(arg2_mode, pc+2, intcode);
        // let _arg3 = decode_arg(arg3_mode, pc+3, intcode);
        match opcode {
            1 => {intcode[raw_arg_3 as usize] = decode_arg1() + decode_arg2(); pc += 4},
            2 => {intcode[raw_arg_3 as usize] = decode_arg1() * decode_arg2(); pc += 4},
            3 => {intcode[raw_arg_1 as usize] = read_int(); pc += 2},
            4 => {write_int(decode_arg1()); pc += 2},
            _ => break,
        };
    }
    intcode[0]
}

#[aoc(day5, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    run_program(&mut local_program)
}

#[aoc(day5, part2)]
pub fn part2(program: &[i32]) -> i32 {
    let mut local_program = program.to_vec();
    local_program[1] = 12;
    local_program[2] = 2;
    run_program(&mut local_program)
}

#[cfg(test)]
mod tests {
    use super::{is_increasing, has_double_number, has_exclusive_double_number};

    #[test]
    fn is_increasing_test() {
        assert_eq!(is_increasing(123456), true);
        assert_eq!(is_increasing(111111), true);
        assert_eq!(is_increasing(111110), false);
        assert_eq!(is_increasing(211111), false);
    }

    #[test]
    fn double_nonexclusive_test() {
        assert_eq!(has_double_number(111111), true);
        assert_eq!(has_double_number(112345), true);
        assert_eq!(has_double_number(123455), true);
        assert_eq!(has_double_number(122345), true);
        assert_eq!(has_double_number(123456), false);
    }

    #[test]
    fn double_exclusive_test() {
        assert_eq!(has_exclusive_double_number(111111), false);
        assert_eq!(has_exclusive_double_number(112345), true);
        assert_eq!(has_exclusive_double_number(123455), true);
        assert_eq!(has_exclusive_double_number(122345), true);
        assert_eq!(has_exclusive_double_number(123456), false);
        assert_eq!(has_exclusive_double_number(113455), true);
        assert_eq!(has_exclusive_double_number(111455), true);
        assert_eq!(has_exclusive_double_number(111555), false);
    }
}
