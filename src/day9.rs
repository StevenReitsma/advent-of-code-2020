use std::fs;

pub fn get_input() -> Vec<usize> {
    let input = fs::read_to_string("input/day9.txt").expect("couldn't read input file");
    return input.lines().map(|x| x.parse().unwrap()).collect();
}

pub fn is_sum_of_previous(previous: &[usize], num: usize) -> bool {
    for a in 0..previous.len() {
        for b in (a + 1)..previous.len() {
            if previous[a] + previous[b] == num {
                return true;
            }
        }
    }

    return false;
}

pub fn a(input: &Vec<usize>) -> usize {
    for x in 25..input.len() {
        let num = input[x];

        if !is_sum_of_previous(&input[x - 25..x], num) {
            return num;
        }
    }

    return 0;
}

pub fn b(input: &Vec<usize>) -> usize {
    let invalid = a(input);

    let mut running_sum = 0;
    let mut numbers = Vec::<usize>::new();

    for num in input {
        running_sum += num;
        numbers.push(*num);

        if running_sum == invalid {
            return numbers.iter().min().unwrap() + numbers.iter().max().unwrap();
        }

        while running_sum > invalid {
            running_sum -= numbers.remove(0);
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input());
        assert_eq!(result, 14360655);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input());
        assert_eq!(result, 1962331);
    }
}
