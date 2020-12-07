use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day5.txt").expect("couldn't read input file");
    return input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();
}

fn get_seat_id(bsp: &String) -> usize {
    let binary = bsp
        .replace("B", "1")
        .replace("R", "1")
        .replace("F", "0")
        .replace("L", "0");

    return usize::from_str_radix(&binary, 2).unwrap();
}

pub fn a(input: Vec<String>) -> Option<usize> {
    return input.iter().map(|s| get_seat_id(s)).max();
}

pub fn b(input: Vec<String>) -> usize {
    let mut seat_ids = input.iter().map(|s| get_seat_id(s)).collect::<Vec<usize>>();
    seat_ids.sort();

    let mut prev = 0;
    for seat_id in seat_ids {
        if prev + 2 == seat_id {
            break;
        }

        prev = seat_id;
    }

    return prev + 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(get_input()).unwrap();
        assert_eq!(result, 976);
    }

    #[test]
    fn example_b() {
        let result = b(get_input());
        assert_eq!(result, 685);
    }
}
