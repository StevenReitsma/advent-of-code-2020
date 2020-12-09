use std::collections::HashMap;
use std::fs;

type BagStructure = HashMap<String, Vec<BagContents>>;

pub struct BagContents {
    name: String,
    count: usize,
}

pub fn get_input() -> BagStructure {
    let input = fs::read_to_string("input/day7.txt")
        .expect("couldn't read input file")
        .replace("bags", "bag")
        .replace(".", "");

    let mut structure = BagStructure::new();

    for line in input.lines() {
        let splits: Vec<&str> = line.split(" contain ").collect();
        let outer_bag = splits[0].to_string();

        let inner_bags = splits[1];
        let inner_bag_split = inner_bags.split(", ");

        let mut all_contents = Vec::<BagContents>::new();

        for inner_bag_string in inner_bag_split {
            if &inner_bag_string[0..2] == "no" {
                break;
            }

            let name_count_split: Vec<&str> = inner_bag_string.split(" ").collect();

            let inner_name = name_count_split[1..name_count_split.len()].join(" ");
            let inner_count: usize = name_count_split[0].parse().unwrap();

            let contents = BagContents {
                name: inner_name,
                count: inner_count,
            };

            all_contents.push(contents);
        }

        structure.insert(outer_bag, all_contents);
    }

    return structure;
}

pub fn rec_a(input: &BagStructure, root: &str) -> isize {
    if root == "shiny gold bag" {
        return 1;
    }

    return input[root]
        .iter()
        .map(|x| rec_a(input, &x.name))
        .any(|x| x == 1) as isize;
}

pub fn a(input: &BagStructure) -> isize {
    return input.keys().fold(-1, |acc, bag| acc + rec_a(&input, bag));
}

pub fn b(input: &BagStructure, root: &str) -> usize {
    return input[root]
        .iter()
        .fold(0, |acc, x| acc + b(input, &x.name) * x.count + x.count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input());
        assert_eq!(result, 289);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input(), "shiny gold bag");
        assert_eq!(result, 30055);
    }
}
