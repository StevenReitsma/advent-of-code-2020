use countmap::CountMap;
use std::fs;

pub fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("input/day10.txt").expect("couldn't read input file");
    return input.lines().map(|x| x.parse().unwrap()).collect();
}

pub fn a(input: &Vec<usize>) -> usize {
    let mut sorted_input = input.clone();
    sorted_input.sort();

    let mut joltage = 0;
    let mut joltage_diffs: CountMap<usize, usize> = CountMap::new();
    for adapter in sorted_input {
        let diff = adapter - joltage;
        joltage = adapter;

        joltage_diffs.insert_or_increment(diff);
    }

    return joltage_diffs.get_count(&1).unwrap() * (joltage_diffs.get_count(&3).unwrap() + 1);
}

pub fn b(input: &Vec<usize>) -> usize {
    let mut sorted_input = input.clone();
    sorted_input.sort();

    let device = *sorted_input.last().unwrap() + 3;
    sorted_input.push(device);

    let mut joltages = vec![0; device + 1];
    joltages[0] = 1;

    for adapter in sorted_input {
        joltages[adapter] = joltages[adapter.saturating_sub(3)..adapter].iter().sum();
    }

    return *joltages.last().unwrap();
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input());
        assert_eq!(result, 2590);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input());
        assert_eq!(result, 226775649501184);
    }
}
