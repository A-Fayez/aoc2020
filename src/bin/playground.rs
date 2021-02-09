fn main() {
    let input = include_str!("inputs/day04-test.txt");
    //let sa = s.ends_with("\n\n").collect::<Vec<&str>>();
    // let ps = input
    //     .lines()
    //     .filter(|line| !line.trim().is_empty())
    //     .collect::<Vec<_>>();

    let ps = input.split("\n\n").collect::<Vec<_>>();
    let z = ps[0].split_whitespace().collect::<Vec<_>>();
    println!("Count: {}", ps.len());
    for line in ps {
        println!("{}", line);
        println!("---------");
    }
    println!("items");
    for i in z {
        println!("{}", i);
    }
}
