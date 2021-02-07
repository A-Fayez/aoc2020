fn main() {
    let a = r"
    ab
    ab
    ab
    "
    .to_string();
    let b = a.repeat(3);
    println!("{}", b);
}
