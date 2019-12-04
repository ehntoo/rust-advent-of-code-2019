#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split("-").map(|x| x.parse().unwrap()).collect()
}

fn has_double_number(i: i32) -> bool {
    let mut divisor = 100000;
    let mut last_digit = i/divisor;
    let mut remainder = i%divisor;
    while divisor > 1 {
        divisor /= 10;
        let next_digit = remainder/divisor;
        if next_digit == last_digit {
            return true;
        }
        last_digit = next_digit;
        remainder = remainder%divisor;
    }
    false
}

fn is_increasing(i: i32) -> bool {
    let mut divisor = 100000;
    let mut last_digit = i/divisor;
    let mut remainder = i%divisor;
    while divisor > 1 {
        divisor /= 10;
        let next_digit = remainder/divisor;
        if next_digit < last_digit {
            return false;
        }
        last_digit = next_digit;
        remainder = remainder%divisor;
    }
    true
}

#[aoc(day4, part1)]
pub fn part1(number_bounds: &[i32]) -> i32 {
    let lower_bound = number_bounds[0];
    let upper_bound = number_bounds[1];
    let mut candidates: Vec<i32> = vec![];
    for i in lower_bound..=upper_bound {
        if has_double_number(i) && is_increasing(i) {
            candidates.push(i);
        }
    }
    candidates.len() as i32
}

#[aoc(day4, part2)]
pub fn part2(_number_bounds: &[i32]) -> i32 {
    0
}
