use modinverse::modinverse;
use std::fs;

pub fn get_input() -> (isize, Vec<isize>) {
    let input = fs::read_to_string("input/day13.txt").expect("couldn't read input file");

    let time = input.lines().nth(0).unwrap().parse().unwrap();
    let buses = input
        .lines()
        .nth(1)
        .unwrap()
        .replace("x,", "0,")
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    return (time, buses);
}

pub fn a(time: isize, buses: &Vec<isize>) -> isize {
    let mut closest_bus = 0;
    let mut waiting = 1.;

    for bus in buses {
        if *bus == 0 {
            continue;
        }

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

// START from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: Vec<isize>, modulii: Vec<isize>) -> Option<isize> {
    let prod = modulii.iter().product::<isize>();

    let mut sum = 0;

    for (residue, modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * modinverse(p, modulus)? * p
    }

    Some(sum % prod)
}
// END from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

pub fn b(buses: &Vec<isize>) -> isize {
    let modulii = buses
        .iter()
        .filter(|x| **x != 0)
        .map(|x| *x)
        .collect();

    let residues = buses
        .iter()
        .enumerate()
        .filter(|&(_, x)| *x != 0)
        .map(|(i, x)| (-(i as isize)).rem_euclid(*x))
        .collect();

    return chinese_remainder(residues, modulii).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(input.0, &input.1);
        assert_eq!(result, 370);
    }

    #[test]
    fn example_b() {
        let input = &get_input();
        let result = b(&input.1);
        assert_eq!(result, 894954360381385);
    }
}
