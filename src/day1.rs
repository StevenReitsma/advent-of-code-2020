use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

pub struct NotFoundError {}

pub fn get_input() -> Result<Vec<isize>, std::num::ParseIntError> {
    let input = fs::read_to_string("input/day1.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn compute(
    input: Vec<isize>,
    target: isize,
    n_elements: isize,
) -> Result<isize, NotFoundError> {
    if n_elements == 1 {
        if input.contains(&target) {
            return Ok(target);
        }

        return Err(NotFoundError {});
    }

    if n_elements == 2 {
        let mut rest: HashSet<isize> = HashSet::new();

        for x in input.iter() {
            if rest.contains(x) {
                return Ok((target - x) * x);
            }

            rest.insert(target - x);
        }

        return Err(NotFoundError {});
    }

    for x in input.iter() {
        let new_target = target - x;
        let recursive = compute(input.clone(), new_target, n_elements - 1);
        match recursive {
            Ok(prev) => return Ok(x * prev),
            Err(_) => continue,
        }
    }

    return Err(NotFoundError {});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = compute(get_input().unwrap(), 2020, 2);
        match result {
            Ok(r) => assert_eq!(r, 866436),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn example_b() {
        let result = compute(get_input().unwrap(), 2020, 3);
        match result {
            Ok(r) => assert_eq!(r, 276650720),
            Err(_) => assert!(false),
        }
    }
}
