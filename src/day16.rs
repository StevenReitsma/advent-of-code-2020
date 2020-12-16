use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
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
    pub fn is_valid(&self, num: isize) -> bool {
        return (num >= self.min1 && num <= self.max1) || (num >= self.min2 && num <= self.max2);
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
            if !ranges.iter().any(|r| r.is_valid(*num)) {
                errors += num;
            }
        }
    }

    return errors;
}

pub fn b(my_ticket: &Ticket, tickets: &Vec<Ticket>, ranges: &Vec<Range>) -> isize {
    // Find all valid tickets
    let mut valid_tickets = Vec::<&Ticket>::new();

    'tickets: for t in tickets {
        for num in t {
            if !ranges.iter().any(|r| r.is_valid(*num)) {
                continue 'tickets;
            }
        }

        valid_tickets.push(t);
    }

    valid_tickets.push(my_ticket);

    // Enumerate all possible orderings
    let mut possibilities = Vec::<HashSet<usize>>::new();

    for position in 0..my_ticket.len() {
        let mut allowed_ranges = HashSet::<usize>::new();

        for (i, range) in ranges.iter().enumerate() {
            let mut numbers = valid_tickets.iter().map(|t| t[position]);
            if numbers.all(|n| range.is_valid(n)) {
                allowed_ranges.insert(i);
            }
        }

        possibilities.push(allowed_ranges);
    }

    let mut assigned = [0; 20];

    // While some position has only one possible field
    while let Some(i) = possibilities.iter().position(|s| s.len() == 1) {
        let v = *possibilities[i].iter().next().unwrap();
        assigned[i] = v;
        // Remove it from all sets of possibilities
        for s in &mut possibilities {
            s.remove(&v);
        }
    }

    return assigned
        .iter()
        .enumerate()
        .filter(|(_, &rule)| rule < 6)  // first six are departure fields
        .map(|(i, _)| my_ticket[i])
        .product();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let inputs = &get_input();
        let result = a(&inputs.1, &inputs.2);
        assert_eq!(result, 20975);
    }

    #[test]
    fn example_b() {
        let inputs = &get_input();
        let result = b(&inputs.0, &inputs.1, &inputs.2);
        assert_eq!(result, 910339449193);
    }
}
