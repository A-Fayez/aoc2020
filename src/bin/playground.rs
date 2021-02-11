fn main() {
    let s = "15cm";
    println!("{}", s.split("cm").next().unwrap());
}
