#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split("-").map(|x| x.parse().unwrap()).collect()
}

fn has_double_number(i: i32, exclusive: bool) -> bool {
    let mut divisor = 100000;
    let mut last_digit = i/divisor;
    let mut remainder = i%divisor;
    let mut recently_seen_double = false;
    let mut ever_seen_double = false;
    while divisor > 1 {
        divisor /= 10;
        let next_digit = remainder/divisor;
        if next_digit == last_digit {
            if exclusive && recently_seen_double {
                return false;
            }
            recently_seen_double = true;
            ever_seen_double = true;
        } else {
            recently_seen_double = false;
        }
        last_digit = next_digit;
        remainder = remainder%divisor;
    }
    ever_seen_double
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
        if has_double_number(i, false) && is_increasing(i) {
            candidates.push(i);
        }
    }
    candidates.len() as i32
}

#[aoc(day4, part2)]
pub fn part2(number_bounds: &[i32]) -> i32 {
    let lower_bound = number_bounds[0];
    let upper_bound = number_bounds[1];
    let mut candidates: Vec<i32> = vec![];
    for i in lower_bound..=upper_bound {
        if has_double_number(i, true) && is_increasing(i) {
            candidates.push(i);
        }
    }
    candidates.len() as i32
}

#[cfg(test)]
mod tests {
    use super::{is_increasing, has_double_number};

    #[test]
    fn is_increasing_test() {
        assert_eq!(is_increasing(123456), true);
        assert_eq!(is_increasing(111111), true);
        assert_eq!(is_increasing(111110), false);
        assert_eq!(is_increasing(211111), false);
    }

    #[test]
    fn double_nonexclusive_test() {
        assert_eq!(has_double_number(111111, false), true);
        assert_eq!(has_double_number(112345, false), true);
        assert_eq!(has_double_number(123455, false), true);
        assert_eq!(has_double_number(122345, false), true);
        assert_eq!(has_double_number(123456, false), false);
    }
    #[test]
    fn double_exclusive_test() {
        assert_eq!(has_double_number(111111, true), false);
        assert_eq!(has_double_number(112345, true), true);
        assert_eq!(has_double_number(123455, true), true);
        assert_eq!(has_double_number(122345, true), true);
        assert_eq!(has_double_number(123456, true), false);
        assert_eq!(has_double_number(113455, true), true);
        assert_eq!(has_double_number(111455, true), false);
    }
}
