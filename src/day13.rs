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
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: Vec<i64>, modulii: Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (residue, modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

// END from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

pub fn b(buses: &Vec<isize>) -> i64 {
    let modulii = buses
        .iter()
        .filter(|x| **x != 0)
        .map(|x| *x as i64)
        .collect();

    let residues = buses
        .iter()
        .enumerate()
        .filter(|&(_, x)| *x != 0)
        .map(|(i, x)| (*x - i as isize) as i64)
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
