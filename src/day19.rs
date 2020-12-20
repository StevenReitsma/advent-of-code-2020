use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn get_input() -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string("input/day19.txt").expect("couldn't read input file");
    let splits = input
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let rules = splits[0].lines().map(|x| x.to_string()).collect();
    let messages = splits[1].lines().map(|x| x.to_string()).collect();

    return (rules, messages);
}

// Convert rules to regex strings
pub fn parse_rules(rules: &Vec<String>, b: bool) -> HashMap<isize, String> {
    lazy_static! {
        static ref RAW_RULE_REGEX: Regex = Regex::new(r"(\d*): (.*)").unwrap();
        static ref NUM_REGEX: Regex = Regex::new(r" (\d{1,3}) ").unwrap();
    }

    // First add all rules to hashmap
    let mut rule_map = rules
        .iter()
        .map(|x| {
            let caps = RAW_RULE_REGEX.captures(x).unwrap();
            (
                caps[1].parse().unwrap(),
                format!(" {} ", caps[2].to_string()),
            )
        })
        .collect::<HashMap<isize, String>>();

    // Modify rules for B
    if b {
        rule_map.insert(8, " 42 + ".to_string());
        rule_map.insert(
            11,
            " 42 31 | 42 {2} 31 {2} | 42 {3} 31 {3} | 42 {4} 31 {4} ".to_string(),
        );
    }

    // Start filling variables until none are found
    let mut all_flat = false;
    while !all_flat {
        let mut new_rule_map = rule_map.clone();
        all_flat = true;

        for (id, rule) in rule_map.iter() {
            let mut new_rule = rule.clone();

            for num in NUM_REGEX.captures_iter(rule) {
                all_flat = false;

                let numnum = &num[0].replace(" ", "").parse().unwrap();
                new_rule = new_rule.replace(
                    &format!(" {} ", numnum),
                    &format!(" ( {} ) ", rule_map[numnum]),
                );
            }

            new_rule_map.insert(*id, new_rule);
        }

        rule_map = new_rule_map.clone();
    }

    // Clean up quotes and spaces
    for (_, rule) in rule_map.iter_mut() {
        *rule = format!("^({})$", rule.replace(" ", "").replace("\"", ""));
    }

    return rule_map;
}

pub fn a(rules: &Vec<String>, messages: &Vec<String>, b: bool) -> usize {
    let regex_rules = parse_rules(rules, b);
    let mut regex_compiled = HashMap::<isize, Regex>::new();

    // Compile all regexes
    for (id, rule) in regex_rules {
        if id == 8 || id == 11 || id == 0 {
            println!("{}: {}", id, rule);
        }
        regex_compiled.insert(id, Regex::new(&rule).unwrap());
    }

    // Find valid messages
    return messages
        .iter()
        .filter(|m| regex_compiled[&0].is_match(m))
        .count();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let input = &get_input();
        let result = a(&input.0, &input.1, false);
        assert_eq!(result, 156);
    }

    #[test]
    fn example_b() {
        let input = &get_input();
        let result = a(&input.0, &input.1, true);
        assert_eq!(result, 363);
    }
}
