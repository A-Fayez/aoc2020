const INPUT: &'static str = include_str!("inputs/day03.txt");
fn main() {
    println!("part1 Answer: {}", count_trees(3, 1, INPUT));
    let part2 = count_trees(3, 1, INPUT)
        * count_trees(1, 1, INPUT)
        * count_trees(5, 1, INPUT)
        * count_trees(7, 1, INPUT)
        * count_trees(1, 2, INPUT);
    println!("part2 Answer: {}", part2);
}

#[test]
fn test_count_trees() {
    let input = include_str!("inputs/day03-test.txt");
    let output = count_trees(1, 2, input);
    assert_eq!(output, 2);
}

fn count_trees(right: usize, down: usize, input: &str) -> usize {
    let mut trees_count = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let max_row = lines.len() - 1;
    let max_col = lines[0].chars().collect::<Vec<_>>().len();
    let mut cur_col = 0;
    let mut cur_row = 0;

    for _ in 0..max_row {
        cur_row = cur_row + down;

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
        if cur_row == max_row {
            break;
        }
    }

    trees_count
}
