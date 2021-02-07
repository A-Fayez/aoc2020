const INPUT: &'static str = include_str!("inputs/day03.txt");
fn main() {
    print!("trees: {}", count_trees(3, 1, INPUT));
}

#[test]
fn test_count_trees() {
    let input = include_str!("inputs/day03-test.txt");
    let output = count_trees(3, 1, input);
    assert_eq!(output, 7);
}

fn count_trees(right: usize, down: usize, input: &str) -> usize {
    let mut trees_count = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let max_row = lines.len() - 1;
    let max_col = lines[0].chars().collect::<Vec<_>>().len();
    println!("max row: {}, max col: {}", max_row, max_col);
    let mut cur_col = 0;

    for i in 0..max_row {
        let cur_row = i + down;

        cur_col = (cur_col + right - 1) % max_col + 1;
        if cur_col % max_col == 0 {
            if lines[cur_row].chars().nth(0).unwrap() == '#' {
                trees_count = trees_count + 1;
            }
        } else {
            if lines[cur_row].chars().nth(cur_col).unwrap() == '#' {
                trees_count = trees_count + 1;
            }
        }
    }
    trees_count
}
