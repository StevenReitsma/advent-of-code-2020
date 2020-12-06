use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day4.txt").expect("couldn't read input file");
    return input.split("\n\n").map(|s| s.to_string()).collect();
}

pub fn has_valid_fields(passport: &String) -> bool {
    let fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for field in fields {
        if !passport.contains(field) {
            return false;
        }
    }

    return true;
}

pub fn has_valid_field_contents(passport: &String) -> bool {
    lazy_static! {
        static ref RE: Vec<Regex> = vec![
            Regex::new(r"(?m)byr:(19[2-9][0-9]|200[0-2])( |$)").unwrap(),
            Regex::new(r"(?m)iyr:(201[0-9]|2020)( |$)").unwrap(),
            Regex::new(r"(?m)eyr:(202[0-9]|2030)( |$)").unwrap(),
            Regex::new(r"(?m)hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)( |$)").unwrap(),
            Regex::new(r"(?m)hcl:(#[0-9a-f]{6})( |$)").unwrap(),
            Regex::new(r"(?m)ecl:(amb|blu|brn|gry|grn|hzl|oth)( |$)").unwrap(),
            Regex::new(r"(?m)pid:([0-9]{9})( |$)").unwrap(),
        ];
    }

    for regex in RE.iter() {
        if !regex.is_match(passport) {
            return false;
        }
    }

    println!("{}\n", passport);

    return true;
}

pub fn a(input: Vec<String>) -> usize {
    let passport_count = input
        .iter()
        .map(|p| has_valid_fields(p))
        .map(|b| b as usize)
        .sum();
    return passport_count;
}

pub fn b(input: Vec<String>) -> usize {
    let passport_count = input
        .iter()
        .map(|p| has_valid_field_contents(p))
        .map(|b| b as usize)
        .sum();
    return passport_count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(get_input());
        assert_eq!(result, 254);
    }

    #[test]
    fn example_b() {
        let result = b(get_input());
        assert_eq!(result, 0);
    }
}
