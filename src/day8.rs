use itertools::Itertools;

const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split("").map(|l| l.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn part1(image: &[i32]) -> i32 {
    let layers = image.chunks(IMG_WIDTH * IMG_HEIGHT);
    let layer_statistics = layers.map(|l| l.iter().group_by(|elt| *elt)
        .into_iter().map(|(k, g)| (k, g.count())));
    0
}
