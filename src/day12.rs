use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone)]
pub struct NavInstruction {
    action: char,
    amount: isize,
}

impl FromStr for NavInstruction {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(NavInstruction {
            action: string.chars().nth(0).unwrap(),
            amount: string[1..].parse()?,
        })
    }
}

pub fn get_input() -> Result<Vec<NavInstruction>, Box<dyn Error>> {
    let input = fs::read_to_string("input/day12.txt").expect("couldn't read input file");
    return Result::from_iter(input.lines().map(|x| x.parse()));
}

pub fn move_ship(x: &mut isize, y: &mut isize, direction: isize, amount: isize) {
    match direction {
        0 => *y -= amount as isize,
        90 => *x += amount as isize,
        180 => *y += amount as isize,
        270 => *x -= amount as isize,
        _ => return,
    }
}

pub fn a(input: &Vec<NavInstruction>) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 90;

    for instruction in input {
        match instruction.action {
            'F' => move_ship(&mut x, &mut y, direction, instruction.amount),
            'N' => move_ship(&mut x, &mut y, 0, instruction.amount),
            'S' => move_ship(&mut x, &mut y, 180, instruction.amount),
            'W' => move_ship(&mut x, &mut y, 270, instruction.amount),
            'E' => move_ship(&mut x, &mut y, 90, instruction.amount),
            'L' => direction = (direction + -instruction.amount).rem_euclid(360),
            'R' => direction = (direction + instruction.amount) % 360,
            _ => return 0,
        }
    }

    return x.abs() + y.abs();
}

pub fn rotate_90(x: isize, y: isize) -> (isize, isize) {
    return (-y, x);
}

pub fn rotate_90_ccw(x: isize, y: isize) -> (isize, isize) {
    return (y, -x);
}

pub fn b(input: &Vec<NavInstruction>) -> isize {
    let mut x_ship = 0;
    let mut y_ship = 0;

    let mut x_waypoint = 10;
    let mut y_waypoint = -1;

    for instruction in input {
        match instruction.action {
            'F' => {
                x_ship += x_waypoint * instruction.amount;
                y_ship += y_waypoint * instruction.amount;
            }
            'N' => y_waypoint -= instruction.amount,
            'S' => y_waypoint += instruction.amount,
            'W' => x_waypoint -= instruction.amount,
            'E' => x_waypoint += instruction.amount,
            'L' => {
                for _ in 0..instruction.amount / 90 {
                    // Rust doesn't support `(x, y) = fn(x, y)` :(
                    let rotated = rotate_90_ccw(x_waypoint, y_waypoint);
                    x_waypoint = rotated.0;
                    y_waypoint = rotated.1;
                }
            }
            'R' => {
                for _ in 0..instruction.amount / 90 {
                    let rotated = rotate_90(x_waypoint, y_waypoint);
                    x_waypoint = rotated.0;
                    y_waypoint = rotated.1;
                }
            }
            _ => return 0,
        }
    }

    return x_ship.abs() + y_ship.abs();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input().unwrap());
        assert_eq!(result, 1956);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input().unwrap());
        assert_eq!(result, 126797);
    }
}
