use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input found")
    }
}
impl Error for ParseError {}

#[derive(Clone)]
pub struct Instruction {
    op: String,
    value: isize,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.{3}) (-\d+|\+\d+)").unwrap();
        }

        let caps = RE.captures(string);

        match caps {
            Some(c) => Ok(Instruction {
                op: c[1].to_string(),
                value: c[2].trim_start_matches('+').parse()?,
            }),
            None => Err(Box::new(ParseError {})),
        }
    }
}

pub struct ComputeResult {
    value: isize,
    finished: bool,
}

pub fn get_input() -> Result<Vec<Instruction>, Box<dyn Error>> {
    let input = fs::read_to_string("input/day8.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn a(input: Vec<Instruction>) -> ComputeResult {
    let mut accumulator = 0;
    let mut stack_pointer: isize = 0;
    let mut counter = HashSet::<isize>::new();

    while !counter.contains(&stack_pointer)
        && stack_pointer < input.len().try_into().unwrap()
        && stack_pointer >= 0
    {
        counter.insert(stack_pointer);

        let instruction = &input[stack_pointer as usize];

        match &instruction.op as &str {
            "nop" => stack_pointer += 1,
            "jmp" => stack_pointer += instruction.value,
            "acc" => {
                stack_pointer += 1;
                accumulator += instruction.value;
            }
            _ => {}
        }
    }

    return ComputeResult {
        value: accumulator,
        finished: stack_pointer >= input.len().try_into().unwrap(),
    };
}

pub fn mutate(instruction: Instruction) -> Instruction {
    let op: &str;

    match &instruction.op as &str {
        "jmp" => op = "nop",
        "nop" => op = "jmp",
        _ => op = "nop",
    }

    Instruction {
        op: op.to_string(),
        value: instruction.value,
    }
}

pub fn b(input: Vec<Instruction>) -> ComputeResult {
    let mut i = 0;
    for instruction in input.clone() {
        if instruction.op == "acc" {
            continue;
        }

        let mut new_input = input.clone();
        new_input[i] = mutate(instruction);

        let result = a(new_input);
        if result.finished {
            return result;
        }

        i += 1;
    }

    return ComputeResult {
        value: 0,
        finished: false,
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(get_input().unwrap());
        assert_eq!(result.value, 1600);
    }

    #[test]
    fn example_b() {
        let result = b(get_input().unwrap());
        assert!(result.finished);
        assert_eq!(result.value, 1543);
    }
}
