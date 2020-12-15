use std::collections::HashMap;

pub fn get_input() -> Vec<isize> {
    return vec![0, 5, 4, 1, 10, 14, 7];
}

pub fn a(input: &Vec<isize>, turns: usize) -> isize {
    let mut map = HashMap::<isize, isize>::new();
    let mut last_num = 0;

    for (i, starting_number) in input.iter().enumerate() {
        map.insert(*starting_number, i as isize + 1);
        last_num = *starting_number;
    }

    for i in input.len()..turns {
        let num;

        if map.contains_key(&last_num) {
            num = i as isize - map[&last_num];
        } else {
            num = 0;
        }

        map.insert(last_num, i as isize);
        last_num = num;
    }

    return last_num;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(input, 2020);
        assert_eq!(result, 203);
    }

    #[test]
    fn example_b() {
        let input = &get_input();
        let result = a(input, 30000000);
        assert_eq!(result, 9007186);
    }
}
