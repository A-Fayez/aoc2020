use std::fmt;

fn main() {
    let contents = include_str!("inputs/day02.txt");
    let mut valid_passwords = (0, 0);

    for line in contents.lines() {
        let t: Vec<&str> = line.split(":").collect();
        let policy = Policy::new(t[0]);
        let policy = policy.as_ref();
        let password = Password(t[1].to_owned());

        if password.is_valid_against_p1(&policy.unwrap()) {
            valid_passwords.0 = valid_passwords.0 + 1;
        }
        if password.is_valid_against_p2(&policy.unwrap()) {
            valid_passwords.1 = valid_passwords.1 + 1;
        }
    }
    println!("Part1 Answer: {}", valid_passwords.0);
    println!("Part2 Answer: {}", valid_passwords.1);
}

#[test]
fn part2_test() {
    let pw = Password("pqhpppwpfmr".to_string());
    let policy = Policy::new("2-4 p");
    assert_eq!(pw.is_valid_against_p2(&policy.unwrap()), true);
}

struct Policy {
    boundaries: (usize, usize),
    letter: String,
}

struct Password(String);

impl Policy {
    fn new(s: &str) -> Result<Self, &'static str> {
        let t: Vec<&str> = s.split_whitespace().collect();
        let letter = t[1].to_owned();
        let boundaries: Vec<&str> = t[0].split("-").collect();
        let boundaries: Vec<usize> = boundaries
            .iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let boundaries: (usize, usize) = (boundaries[0], boundaries[1]);
        Ok(Self { boundaries, letter })
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Password {
    fn is_valid_against_p1(&self, policy: &Policy) -> bool {
        let number_of_ocs = self.0.matches(&policy.letter).count();
        number_of_ocs >= policy.boundaries.0 && number_of_ocs <= policy.boundaries.1
    }
    fn is_valid_against_p2(&self, policy: &Policy) -> bool {
        let pos = policy.boundaries;
        let letter = policy.letter.chars().next().unwrap();

        let a = self.0.chars().nth(pos.0 - 1) == Some(letter);
        let b = self.0.chars().nth(pos.1 - 1) == Some(letter);
        a ^ b
    }
}
