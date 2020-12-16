use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

type Ticket = Vec<isize>;

pub struct Range {
    name: String,
    min1: isize,
    min2: isize,
    max1: isize,
    max2: isize,
}

impl Range {
    pub fn is_invalid(&self, num: isize) -> bool {
        return (num >= self.max1 && num > self.max2) || (num < self.min1 && num < self.min2);
    }
}

pub fn get_input() -> (Ticket, Vec<Ticket>, Vec<Range>) {
    lazy_static! {
        static ref RANGE_RE: Regex = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
        static ref TICKET_RE: Regex = Regex::new(r"(\d*,){19}(\d*)").unwrap();
    }

    let input = fs::read_to_string("input/day16.txt").expect("couldn't read input file");
    let splits = input
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut ranges = Vec::<Range>::new();
    let mut tickets = Vec::<Ticket>::new();

    // First split is a list of ticketes
    for range_string in splits[0].lines() {
        let fields = RANGE_RE.captures(range_string).unwrap();

        ranges.push(Range {
            name: fields[1].parse().unwrap(),
            min1: fields[2].parse().unwrap(),
            max1: fields[3].parse().unwrap(),
            min2: fields[4].parse().unwrap(),
            max2: fields[5].parse().unwrap(),
        });
    }

    // Second split is my ticket
    let my_ticket = TICKET_RE.captures(&splits[1]).unwrap()[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // Third split is nearby tickets
    for ticket_string in splits[2].lines() {
        if ticket_string.contains("nearby tickets:") {
            continue;
        }
        let fields = TICKET_RE.captures(ticket_string).unwrap()[0]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        tickets.push(fields);
    }

    return (my_ticket, tickets, ranges);
}

pub fn a(tickets: &Vec<Ticket>, ranges: &Vec<Range>) -> isize {
    let mut errors = 0;

    for t in tickets {
        for num in t {
            if ranges.iter().all(|r| r.is_invalid(*num)) {
                errors += num;
            }
        }
    }

    return errors;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let inputs = &get_input();
        let result = a(&inputs.1, &inputs.2);
        assert_eq!(result, 0);
    }
}
