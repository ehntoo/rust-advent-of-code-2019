use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|l| l.split(',').map(|m| m.to_string()).collect()).collect()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

pub fn advance_position(point: &Point, dir: &str) -> Point {
    match dir {
        "U" => Point {x: point.x, y: point.y+1},
        "D" => Point {x: point.x, y: point.y-1},
        "L" => Point {x: point.x-1, y: point.y},
        "R" => Point {x: point.x+1, y: point.y},
        _ => point.to_owned()
    }
}

pub fn check_collisions(colliding_point: Point, path: &Vec<String>) -> bool {
    let mut current_point = Point {x: 0, y: 0};
    for motion in path {
        let direction = &motion[0..1];
        let distance: i32 = motion[1..].parse().unwrap();
        for _ in 0..distance {
            current_point = advance_position(&current_point, direction);
            if current_point == colliding_point {
                return true;
            }
        }
    }
    return false;
}

#[aoc(day3, part1)]
pub fn part1(moves: &Vec<Vec<String>>) -> i32 {
    let path_one = &moves[0];
    let path_two = &moves[1];
    let mut coord_one = Point {x: 0, y: 0};
    let mut collisions = HashSet::new();
    for motion in path_one {
        let direction = &motion[0..1];
        let distance: i32 = motion[1..].parse().unwrap();
        for _ in 0..distance {
            coord_one = advance_position(&coord_one, direction);
            if check_collisions(coord_one, path_two) {
                collisions.insert(coord_one);
            }
        }
    }
    collisions.iter().map(|p| p.x.abs() + p.y.abs()).min().unwrap()
}

#[aoc(day3, part2)]
pub fn part2(_moves: &Vec<Vec<String>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample1() {
        assert_eq!(part1(&vec![vec!["R8","U5","L5","D3"].iter().map(|s| s.to_string()).collect(), vec!["U7","R6","D4","L4"].iter().map(|s| s.to_string()).collect()]), 6);
    }
}
