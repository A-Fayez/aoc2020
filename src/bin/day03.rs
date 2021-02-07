const INPUT: &'static str = include_str!("inputs/day03.txt");
fn main() {
    let mut trees_count = 0;
    let right = 3;
    let down = 1;
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let max_row = lines.len() - 1;
    let max_col = lines[0].chars().collect::<Vec<_>>().len() - 1;

    let mut cur_col = 0;

    for i in 0..max_row {
        let cur_row = i + down;
        cur_col = cur_col + right;

        println!("curr col: {}", cur_col);
        println!(
            "line index: {}, char index: {}, char: {}",
            cur_row,
            cur_col,
            lines[cur_row]
                .chars()
                .nth((cur_col - 1) % max_col + 1)
                .unwrap()
        );
        if lines[cur_row]
            .chars()
            .nth((cur_col - 1) % max_col + 1)
            .unwrap()
            == '#'
        {
            trees_count = trees_count + 1;
        }
    }
    println!("Trees Count: {}", trees_count);
}
