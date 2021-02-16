use itertools::Itertools;

fn main() {
    const INPUT: &'static str = include_str!("inputs/day06.txt");

    let part1_answer = INPUT.split("\n\n").flat_map(|l| l.lines()).unique().count();

}
