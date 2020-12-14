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

pub fn a(input: &Vec<String>) -> isize {
    lazy_static! {
        static ref MaskRE: Regex = Regex::new(r"mask = (.*)").unwrap();
        static ref MemRE: Regex = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    }

    let mut mask: String = String::from("");
    let mut memmap = HashMap::<isize, isize>::new();

    for instruction in input {
        if instruction.starts_with("mask") {
            mask = MaskRE.captures(instruction).unwrap()[1].to_string();
            continue;
        }

        let caps = MemRE.captures(instruction).unwrap();
        let address: isize = caps[1].parse().unwrap();
        let mem: isize = caps[2].parse().unwrap();

        set_address(&mut memmap, address, mem, &mask);
    }

    return memmap.values().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(input);
        assert_eq!(result, 370);
    }
}
