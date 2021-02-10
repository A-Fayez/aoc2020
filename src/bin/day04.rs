//use anyhow::Result;
use parse_display::{Display, FromStr};
use std::{fmt, str::FromStr};

const INPUT: &'static str = include_str!("inputs/day04.txt");
const TEST_INPUT: &'static str = include_str!("inputs/day04-test.txt");
fn main() {
    let valid_passports = INPUT
        .split("\n\n")
        .map(|p_str| Passport::from_str(p_str))
        .filter(|p| p.as_ref().unwrap().is_valid())
        .count();
    println!("valid passports: {}", valid_passports);
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
}

impl Passport {
    fn has_cid(&self) -> bool {
        self.fields.iter().filter(|&f| f.is_cid()).count() == 1
    }

    fn is_valid(&self) -> bool {
        let number_of_fields = self.fields.len();

        match number_of_fields {
            0..=6 => false,
            7..=8 => self.has_cid(),
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
    assert_eq!(num_of_passports, 4);
}

#[test]
fn test_field_is_cid_method() {
    let f = "cid:227";
    let f = f.parse::<Field>().unwrap();
    assert!(f.is_cid());
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

    assert!(p.unwrap().is_valid());

    let p = Passport::from_str(
        "
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
    );

    assert!(!p.unwrap().is_valid());
}
