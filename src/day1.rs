#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1, map)]
pub fn solve_part_1(module_masses: &[i32]) -> i32 {
    module_masses.iter().map(|i| (i/3)-2).sum()
}

#[aoc(day1, part1, fold)]
pub fn solve_part_1_fold(module_masses: &[i32]) -> i32 {
    module_masses.iter().fold(0, |acc, i| acc + ((i/3)-2))
}

#[aoc(day1, part2)]
pub fn solve_part_2(module_masses: &[i32]) -> i32 {
    module_masses.iter().map(|mass| {
        let mut mass_remainder = (mass/3)-2;
        let mut fuel_mass_total = 0;
        while mass_remainder > 0 {
            fuel_mass_total += mass_remainder;
            mass_remainder = (mass_remainder/3)-2;
        }
        fuel_mass_total
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::solve_part_1_fold as part1;
    use super::solve_part_2 as part2;

    #[test]
    fn sample1() {
        assert_eq!(part1(&vec![12]), 2);
        assert_eq!(part1(&vec![14]), 2);
        assert_eq!(part1(&vec![1969]), 654);
        assert_eq!(part1(&vec![100756]), 33583);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&vec![0]), 0);
        assert_eq!(part2(&vec![1]), 0);
        assert_eq!(part2(&vec![12]), 2);
        assert_eq!(part2(&vec![14]), 2);
        assert_eq!(part2(&vec![1969]), 966);
        assert_eq!(part2(&vec![100756]), 50346);
    }
}
