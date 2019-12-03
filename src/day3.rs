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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Collision {
    point: Point,
    steps: i32
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

pub fn check_collisions(colliding_point: Point, path: &Vec<String>) -> i32 {
    let mut current_point = Point {x: 0, y: 0};
    let mut steps = 0;
    for motion in path {
        let direction = &motion[0..1];
        let distance: i32 = motion[1..].parse().unwrap();
        for _ in 0..distance {
            steps += 1;
            current_point = advance_position(&current_point, direction);
            if current_point == colliding_point {
                return steps;
            }
        }
    }
    return -1;
}

pub fn collide_paths(path_one: &Vec<String>, path_two: &Vec<String>) -> HashSet<Collision> {
    let mut coord_one = Point {x: 0, y: 0};
    let mut collisions = HashSet::new();
    let mut steps = 0;
    for motion in path_one {
        let direction = &motion[0..1];
        let distance: i32 = motion[1..].parse().unwrap();
        for _ in 0..distance {
            steps += 1;
            coord_one = advance_position(&coord_one, direction);
            let collision_steps = check_collisions(coord_one, path_two);
            if collision_steps > 0{
                collisions.insert(Collision {point: coord_one, steps: steps + collision_steps});
            }
        }
    }
    collisions
}

#[aoc(day3, part1)]
pub fn part1(moves: &Vec<Vec<String>>) -> i32 {
    let path_one = &moves[0];
    let path_two = &moves[1];
    let collisions = collide_paths(path_one, path_two);
    collisions.iter().map(|c| c.point.x.abs() + c.point.y.abs()).min().unwrap()
}

#[aoc(day3, part2)]
pub fn part2(moves: &Vec<Vec<String>>) -> i32 {
    let path_one = &moves[0];
    let path_two = &moves[1];
    let collisions = collide_paths(path_one, path_two);
    // collisions.iter().map(|c| c.point.x.abs() + c.point.y.abs()).min().unwrap()
    collisions.iter().min_by_key(|c| c.steps).unwrap().steps
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample1() {
        assert_eq!(part1(&vec![vec!["R8","U5","L5","D3"].iter().map(|s| s.to_string()).collect(), vec!["U7","R6","D4","L4"].iter().map(|s| s.to_string()).collect()]), 6);
    }
}
