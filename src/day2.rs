use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input found")
    }
}
impl Error for ParseError {}

pub struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+) (.?): (.*)").unwrap();
        }

        let caps = RE.captures(string);

        match caps {
            Some(c) => {
                Ok(Password {
                    min: c[1].parse()?,
                    max: c[2].parse()?,
                    letter: c[3].parse()?,
                    password: c[4].to_string(),
                })
            },
            None => Err(Box::new(ParseError{}))
        }
    }
}

pub fn get_input() -> Result<Vec<Password>, Box<dyn Error>> {
    let input = fs::read_to_string("input/day2.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn compute(input: Vec<Password>) -> usize {
    let mut count: usize = 0;
    for x in input {
        let n_matches = x.password.matches(x.letter).count();
        if n_matches <= x.max && n_matches >= x.min {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute(get_input().unwrap());
        assert_eq!(result, 614);
    }
}
