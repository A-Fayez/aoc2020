use std::fs;
fn main() {
    let numbers_str = fs::read_to_string("inputs/day01.txt").unwrap();
    let numbers: Vec<u32> = numbers_str
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let size = numbers.len();
    for (i, v) in numbers.iter().enumerate() {
        if v.to_owned() > 2020 {
            continue;
        }

        for j in (i + 1)..size {
            if v + numbers[j] == 2020 {
                println!("Part1:Answer: {}", v * numbers[j])
            }

            for k in (j + 1)..size {
                if v + numbers[j] + numbers[k] == 2020 {
                    println!("Part2:Answer: {}", v * numbers[j] * numbers[k])
                }
            }
        }
    }
}
