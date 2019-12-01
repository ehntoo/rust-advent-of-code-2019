#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            l.parse().unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part_1(input: &[u32]) -> u32 {
    input.iter().map(|i| (i/3)-2).sum()
}
