#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(intcode: &[i32]) -> i32 {
    let mut local_program = intcode.to_vec();
    local_program[1] = 12;
    local_program[2] = 2;
    let mut pc = 0;
    loop {
        let opcode = local_program[pc];
        let src_1 = local_program[pc + 1] as usize;
        let src_2 = local_program[pc + 2] as usize;
        let dst = local_program[pc + 3] as usize;
        match opcode {
            1 => local_program[dst] = local_program[src_1] + local_program[src_2],
            2 => local_program[dst] = local_program[src_1] * local_program[src_2],
            _ => break,
        };
        pc += 4;
    }
    local_program[0]
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample1() {
        assert_eq!(part1(&vec![1,9,10,3,2,3,11,0,99,30,40,50]), 3500);
    }
}
