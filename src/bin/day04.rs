use parse_display::{Display, FromStr};
use std::fmt;

const INPUT: &'static str = include_str!("inputs/day04.txt");
fn main() {
    // let passports = INPUT
    //     .split("\n\n")
    //     .map(|p_str| {
    //         let fields = p_str
    //             .split_whitespace()
    //             .map(|f| f.parse::<Field>().unwrap())
    //             .collect::<Vec<_>>();

    //         Passport { fields }
    //     })
    //     .collect::<Vec<_>>();

    // println!("num of passports: {}", passports.len());

    let valid_passports = INPUT
        .split("\n\n")
        .map(|p_str| {
            let fields = p_str
                .split_whitespace()
                .map(|f| f.parse::<Field>().unwrap())
                .collect::<Vec<_>>();

            Passport { fields }
        })
        .filter(|f| f.is_valid())
        .count();

    println!("valids: {}", valid_passports);
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
#[display(style = "none")]
enum FieldType {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    cid,
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
            FieldType::cid => true,
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
            7 => self.has_cid(),
            8 => true,
            _ => false,
        }
    }
}
