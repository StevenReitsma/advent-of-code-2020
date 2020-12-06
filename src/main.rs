mod day1;
mod day2;
use std::time::Instant;

fn main() {
    // Day 1 - A
    let start = Instant::now();
    let result = day1::compute(day1::get_input().unwrap(), 2020, 2);
    let end = Instant::now();

    match result {
        Ok(r) => println!("Result found in {:?}: {}", end - start, r),
        Err(_) => println!("Couldn't find result"),
    }

    // Day 1 - B
    let start = Instant::now();
    let result = day1::compute(day1::get_input().unwrap(), 2020, 3);
    let end = Instant::now();

    match result {
        Ok(r) => println!("Result found in {:?}: {}", end - start, r),
        Err(_) => println!("Couldn't find result"),
    }

    // Day 2 - A
    let start = Instant::now();
    let result = day2::compute(day2::get_input().unwrap());
    let end = Instant::now();

    println!("Result found in {:?}: {}", end - start, result)
}
