use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day4.txt").expect("couldn't read input file");
    return input.split("\n\n").map(|s| s.to_string()).collect();
}

pub fn a(input: Vec<String>) -> usize {
    let fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut passport_count = 0;

    'passports: for passport in input {
        for field in fields {
            if !passport.contains(field) {
                continue 'passports
            }
        }

        passport_count += 1;
    }

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
}
