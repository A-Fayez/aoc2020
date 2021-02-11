//use anyhow::Result;
use parse_display::{Display, FromStr};
use raster::Color;
use std::{fmt, str::FromStr};

const INPUT: &'static str = include_str!("inputs/day04.txt");
fn main() {
    println!("part1 -> valid passports: {}", part1(INPUT));
    println!("part2 -> valid passports: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p_str| Passport::from_str(p_str))
        .filter(|p| p.as_ref().unwrap().is_valid_part1())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p_str| Passport::from_str(p_str))
        .filter(|p| p.as_ref().unwrap().is_valid())
        .count()
}
struct Passport {
    fields: Vec<Field>,
}

#[derive(Display, FromStr, Debug)]
#[display("{_type}:{value}")]
struct Field {
    _type: FieldType,
    value: String,
}
#[derive(Display, FromStr, Debug)]
#[display(style = "lowercase")]
enum FieldType {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_value = String::new();
        for field in &self.fields {
            str_value.push_str(&field.to_string());
        }
        write!(f, "{}", str_value)
    }
}

impl Field {
    fn is_cid(&self) -> bool {
        match self._type {
            FieldType::Cid => true,
            _ => false,
        }
    }

    fn is_valid(&self) -> bool {
        match self._type {
            FieldType::Byr => {
                let value = self.value.parse::<u32>().unwrap();
                value.to_string().chars().count() == 4 && value >= 1920 && value <= 2002
            }
            FieldType::Iyr => {
                let value = self.value.parse::<u32>().unwrap();
                value.to_string().chars().count() == 4 && value >= 2010 && value <= 2020
            }
            FieldType::Eyr => {
                let value = self.value.parse::<u32>().unwrap();
                value.to_string().chars().count() == 4 && value >= 2020 && value <= 2030
            }
            FieldType::Hgt => {
                let value = &self.value;

                if value.ends_with("cm") {
                    let value = value.split("cm").next().unwrap().parse::<u32>().unwrap();
                    value >= 150 && value <= 193
                } else if value.ends_with("in") {
                    let value = value.split("in").next().unwrap().parse::<u32>().unwrap();
                    value >= 59 && value <= 76
                } else {
                    false
                }
            }
            FieldType::Hcl => {
                let color = Color::hex(&self.value);
                color.is_ok()
            }
            FieldType::Ecl => {
                let value = self.value.as_str();
                match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            }
            FieldType::Pid => self.value.chars().count() == 9,
            FieldType::Cid => true,
        }
    }
}

impl Passport {
    fn has_cid(&self) -> bool {
        self.fields.iter().filter(|&f| f.is_cid()).count() == 1
    }

    fn is_valid_part1(&self) -> bool {
        let number_of_fields = self.fields.len();

        match number_of_fields {
            0..=6 => false,
            7 => !self.has_cid(),
            8 => true,
            _ => false,
        }
    }
    fn is_valid(&self) -> bool {
        let number_of_fields = self.fields.len();

        match number_of_fields {
            0..=6 => false,
            7 => !self.has_cid() && self.fields.iter().all(|f| f.is_valid()),
            8 => self.fields.iter().all(|f| f.is_valid()),
            _ => false,
        }
    }
}

impl FromStr for Passport {
    type Err = parse_display::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split_whitespace()
            .map(|f| f.parse::<Field>())
            .collect::<Result<_, _>>()?;

        Ok(Passport { fields })
    }
}

#[test]
fn test_number_of_passports_and_passport_parsing() {
    const TEST_INPUT: &'static str = include_str!("inputs/day04-test.txt");
    let num_of_passports = TEST_INPUT
        .split("\n\n")
        .map(|p_str| {
            let fields = p_str
                .split_whitespace()
                .map(|f| f.parse::<Field>().unwrap())
                .collect::<Vec<_>>();
            Passport { fields }
        })
        .count();
    assert_eq!(num_of_passports, 10);
}

#[test]
fn test_field_is_cid_method() {
    let f = "cid:227";
    let f = f.parse::<Field>().unwrap();
    assert!(f.is_cid());
    let f = "byr:1996";
    let f = f.parse::<Field>().unwrap();
    assert!(!f.is_cid());
}

#[test]
fn test_passport_parsing() {
    let p = Passport::from_str(
        r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    assert_eq!(p.unwrap().fields.len(), 8);

    let p = Passport::from_str(
        r"hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
    );
    assert_eq!(p.unwrap().fields.len(), 7);
}

#[test]
fn test_passport_has_cid() {
    let p = Passport::from_str(
        r"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    assert!(p.unwrap().has_cid());

    let p = Passport::from_str(
        r"
hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm",
    );

    assert!(!p.unwrap().has_cid());
}

#[test]
fn test_passport_is_vaild_method() {
    let p = Passport::from_str(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm",
    );

    assert!(p.unwrap().is_valid_part1());

    let p = Passport::from_str(
        "
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
    );

    assert!(!p.unwrap().is_valid_part1());
}
