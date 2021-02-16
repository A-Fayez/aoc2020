fn main() {
    const INPUT: &'static str = include_str!("inputs/day05.txt");
    let highest_id = INPUT
        .lines()
        .map(|x| get_row_and_col(x))
        .map(|x| x.0 * 8 + x.1)
        .max()
        .unwrap();

    println!("max: {}", highest_id)
}

fn get_row_and_col(s: &str) -> (isize, isize) {
    let (row, col) = s.split_at(7);
    let row = row.replace("F", "0").replace("B", "1");
    let col = col.replace("L", "0").replace("R", "1");
    let row = isize::from_str_radix(&row, 2).unwrap();
    let col = isize::from_str_radix(&col, 2).unwrap();

    (row, col)
}
