use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input/day14.txt").expect("couldn't read input file");
    return input.lines().map(|x| x.to_string()).collect();
}

pub fn set_address(memmap: &mut HashMap<isize, isize>, address: isize, mem: isize, mask: &String) {
    let ormask = isize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let andmask = isize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();

    let masked_mem = (mem | ormask) & andmask;
    memmap.insert(address, masked_mem);
}

fn set_all_addresses(
    memmap: &mut HashMap<isize, isize>,
    address: &String,
    mem: isize,
    mask: &String,
    position: usize,
) {
    if position == mask.len() {
        memmap.insert(isize::from_str_radix(address, 2).unwrap(), mem);
        return;
    }

    if mask.chars().nth(position).unwrap() == 'X' {
        let mut characters: Vec<char> = address.chars().collect();
        characters[position] = '0';

        let fill_0 = &characters.iter().collect();

        characters[position] = '1';

        let fill_1 = &characters.iter().collect();

        set_all_addresses(memmap, fill_0, mem, mask, position + 1);
        set_all_addresses(memmap, fill_1, mem, mask, position + 1);
    } else {
        set_all_addresses(memmap, address, mem, mask, position + 1);
    }
}

pub fn set_masked_address(
    memmap: &mut HashMap<isize, isize>,
    address: isize,
    mem: isize,
    mask: &String,
) {
    let ormask = isize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let masked_address = address | ormask;

    let binary_string = format!("{:036b}", masked_address);
    set_all_addresses(memmap, &binary_string, mem, mask, 0);
}

pub fn process(
    input: &Vec<String>,
    address_fn: fn(&mut HashMap<isize, isize>, isize, isize, &String),
) -> isize {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"mask = (.*)").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    }

    let mut mask: String = String::from("");
    let mut memmap = HashMap::<isize, isize>::new();

    for instruction in input {
        if instruction.starts_with("mask") {
            mask = MASK_RE.captures(instruction).unwrap()[1].to_string();
            continue;
        }

        let caps = MEM_RE.captures(instruction).unwrap();
        let address: isize = caps[1].parse().unwrap();
        let mem: isize = caps[2].parse().unwrap();

        address_fn(&mut memmap, address, mem, &mask);
    }

    return memmap.values().sum();
}

pub fn a(input: &Vec<String>) -> isize {
    return process(input, set_address);
}

pub fn b(input: &Vec<String>) -> isize {
    return process(input, set_masked_address);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(input);
        assert_eq!(result, 11501064782628);
    }

    #[test]
    fn example_b() {
        let input = &get_input();
        let result = b(input);
        assert_eq!(result, 5142195937660);
    }
}
