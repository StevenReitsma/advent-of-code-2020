use std::collections::HashSet;
use countmap::CountMap;
use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day6.txt").expect("couldn't read input file");
    return input.split("\n\n").map(|s| s.to_string()).collect();
}

pub fn a(input: Vec<String>) -> usize {
    let mut answer = 0;

    for group in input {
        let mut set = HashSet::new();

        for character in group.chars().filter(|c| !c.is_whitespace()) {
            set.insert(character);
        }

        answer += set.len();
    }

    return answer;
}

pub fn b(input: Vec<String>) -> usize {
    let mut answer = 0;

    for group in input {
        let group_size = group.split("\n").collect::<Vec<&str>>().len();
        let mut map: CountMap<char, usize> = CountMap::new();

        for character in group.chars().filter(|c| !c.is_whitespace()) {
            map.insert_or_increment(character);
        }

        answer += map.values().filter(|x| **x == group_size).count();
    }

    return answer;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(get_input());
        assert_eq!(result, 6782);
    }

    #[test]
    fn example_b() {
        let result = b(get_input());
        assert_eq!(result, 3596);
    }
}
