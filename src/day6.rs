pub struct Orbit {
    body: String,
    satellite: String,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
    input.lines().map(|l| {
        let mut parts = l.split(")");
        Orbit {
            body: parts.next().unwrap().to_string(),
            satellite: parts.next().unwrap().to_string(),
        }
    }).collect()
}

fn count_orbits(root: &Orbit, orbit_catalog: &[Orbit], total_orbits: i32) -> i32 {
    let sub_orbits = orbit_catalog.iter().filter(|o| o.body == root.satellite);
    let mut local_orbit_total = total_orbits;
    for o in sub_orbits {
        local_orbit_total += count_orbits(o, orbit_catalog, total_orbits + 1);
    }
    local_orbit_total
}

#[aoc(day6, part1)]
pub fn part1(orbit_catalog: &[Orbit]) -> i32 {
    let roots: Vec<&Orbit> = orbit_catalog.iter().filter(|o| o.body == "COM").collect();
    let mut total_orbits = 0;
    for root in roots {
        total_orbits += count_orbits(root, orbit_catalog, 1);
    }
    total_orbits
}

#[aoc(day6, part2)]
pub fn part2(_orbit_catalog: &[Orbit]) -> i32 {
    0
}
