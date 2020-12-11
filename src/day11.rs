use ndarray::Array2;
use std::fs;

pub fn get_input() -> Result<Array2<char>, ndarray::ShapeError> {
    let input = fs::read_to_string("input/day11.txt").expect("couldn't read input file");
    return Array2::from_shape_vec(
        (input.lines().count(), input.lines().nth(0).unwrap().len()),
        input.replace("\n", "").chars().collect(),
    );
}

pub fn neighbours(input: &Array2<char>, x: usize, y: usize) -> Vec<char> {
    let mut vec = Vec::<char>::new();

    for xx in x.saturating_sub(1)..x + 2 {
        for yy in y.saturating_sub(1)..y + 2 {
            if (xx == x && yy == y) || xx >= input.shape()[0] || yy >= input.shape()[1] {
                continue;
            }

            vec.push(input[[xx, yy]]);
        }
    }

    return vec;
}

pub fn epoch(input: &Array2<char>) -> Array2<char> {
    let mut copy = input.clone();

    for ((x, y), value) in copy.indexed_iter_mut() {
        if *value == 'L'
            && neighbours(&input, x, y)
                .iter()
                .all(|x| *x == 'L' || *x == '.')
        {
            *value = '#';
        } else if *value == '#'
            && neighbours(&input, x, y)
                .iter()
                .filter(|x| **x == '#')
                .count()
                >= 4
        {
            *value = 'L';
        }
    }

    return copy;
}

pub fn a(input: &Array2<char>) -> usize {
    let mut copy = input.clone();

    loop {
        println!("{}", "iteration");
        let new_copy = epoch(&copy);

        // Check if converged
        if copy == new_copy {
            break;
        }

        copy = new_copy;
    }

    return copy.iter().filter(|x| **x == '#').count();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input().unwrap());
        assert_eq!(result, 0);
    }
}
