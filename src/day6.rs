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

fn find_path_to_com<'a>(orbit: &'a Orbit, orbit_catalog: &'a [Orbit]) -> Vec<&'a Orbit> {
    let mut orbital_path = vec![orbit];
    let mut next_orbit = orbit_catalog.iter().find(|o| o.satellite == orbit.body).unwrap();
    while next_orbit.body != "COM" {
        orbital_path.push(next_orbit);
        next_orbit = orbit_catalog.iter().find(|o| o.satellite == next_orbit.body).unwrap();
    }
    orbital_path
}

#[aoc(day6, part2)]
pub fn part2(orbit_catalog: &[Orbit]) -> i32 {
    let my_orbit = orbit_catalog.iter().find(|o| o.satellite == "YOU").unwrap();
    let santa_orbit = orbit_catalog.iter().find(|o| o.satellite == "SAN").unwrap();

    let my_path = find_path_to_com(my_orbit, orbit_catalog);
    let santa_path = find_path_to_com(santa_orbit, orbit_catalog);

    for (my_i, e) in my_path.iter().enumerate() {
        match santa_path.iter().position(|o| o.body == e.body) {
            Some(santa_i) => return (my_i + santa_i) as i32,
            _ => ()
        }
    }
    -1
}
