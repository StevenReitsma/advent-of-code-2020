use std::fs;

pub fn get_input() -> (isize, Vec<isize>) {
    let input = fs::read_to_string("input/day13.txt").expect("couldn't read input file");

    let time = input.lines().nth(0).unwrap().parse().unwrap();
    let buses = input
        .lines()
        .nth(1)
        .unwrap()
        .replace("x,", "")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    return (time, buses);
}

pub fn a(time: isize, buses: &Vec<isize>) -> isize {
    let mut closest_bus = 0;
    let mut waiting = 1.;

    for bus in buses {
        let ratio = time as f32 / *bus as f32;
        let remainder = ratio.ceil() - ratio;

        if remainder < waiting {
            waiting = remainder;
            closest_bus = *bus;
        }
    }

    let minutes = (waiting * closest_bus as f32).ceil() as isize;

    return minutes * closest_bus;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(input.0, &input.1);
        assert_eq!(result, 0);
    }
}
