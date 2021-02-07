use std::fmt;

fn main() {
    let contents = include_str!("inputs/day02.txt");
    let mut valid_passwords = (0, 0);

    for line in contents.lines() {
        let t: Vec<&str> = line.split(": ").collect();
        let policy = Policy::new(t[0].to_string());
        let password = Password(t[1].to_string());
        if password.part1_is_valid_against(&policy) {
            valid_passwords.0 = valid_passwords.0 + 1;
        }
        if password.part2_is_valid_against(&policy) {
            valid_passwords.1 = valid_passwords.1 + 1;
        }
    }
    println!("Part1 Answer: {}", valid_passwords.0);
    println!("Part2 Answer: {}", valid_passwords.1);
}

#[test]
fn part2_test() {
    let pw = Password("pqhpppwpfmr".to_string());
    let policy = Policy::new("2-4 p".to_string());
    assert_eq!(pw.part2_is_valid_against(&policy), true);
}

struct Policy {
    boundaries: (usize, usize),
    letter: char,
}

struct Password(String);

impl Policy {
    fn new(s: String) -> Self {
        let t: Vec<&str> = s.split_whitespace().collect();
        let letter = t[1].to_string().chars().next().unwrap();
        let boundaries: Vec<_> = t[0]
            .split("-")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let boundaries: (usize, usize) = (boundaries[0], boundaries[1]);
        Self { boundaries, letter }
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Password {
    fn part1_is_valid_against(&self, policy: &Policy) -> bool {
        let number_of_ocs = self.0.matches(policy.letter).count();
        number_of_ocs >= policy.boundaries.0 && number_of_ocs <= policy.boundaries.1
    }
    fn part2_is_valid_against(&self, policy: &Policy) -> bool {
        let pos = policy.boundaries;
        let letter = policy.letter;

        let a = self.0.chars().nth(pos.0 - 1) == Some(letter);
        let b = self.0.chars().nth(pos.1 - 1) == Some(letter);
        a ^ b
    }
}
