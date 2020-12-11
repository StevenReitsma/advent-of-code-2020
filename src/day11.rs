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

pub fn line_of_sight_neighbours(input: &Array2<char>, x: usize, y: usize) -> Vec<char> {
    let mut vec = Vec::<char>::new();

    let directions: Vec<[isize; 2]> = vec![
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    for direction in directions {
        let mut xx = x as isize + direction[0];
        let mut yy = y as isize + direction[1];

        while xx >= 0 && yy >= 0 && xx < input.shape()[0] as isize && yy < input.shape()[1] as isize {
            let value = input[[xx as usize, yy as usize]];
            if value == '#' {
                vec.push('#');
                break;
            } else if value == 'L' {
                break;
            }

            xx += direction[0];
            yy += direction[1];
        }
    }

    return vec;
}

pub fn epoch(
    input: &Array2<char>,
    occupancy: usize,
    neighbour_fn: fn(&Array2<char>, usize, usize) -> Vec<char>,
) -> Array2<char> {
    let mut copy = input.clone();

    for ((x, y), value) in copy.indexed_iter_mut() {
        if *value == 'L'
            && neighbour_fn(&input, x, y)
                .iter()
                .all(|z| *z == 'L' || *z == '.')
        {
            *value = '#';
        } else if *value == '#'
            && neighbour_fn(&input, x, y)
                .iter()
                .filter(|z| **z == '#')
                .count()
                >= occupancy
        {
            *value = 'L';
        }
    }

    return copy;
}

pub fn converge(
    input: &Array2<char>,
    occupancy: usize,
    neighbour_fn: fn(&Array2<char>, usize, usize) -> Vec<char>,
) -> usize {
    let mut copy = input.clone();

    loop {
        let new_copy = epoch(&copy, occupancy, neighbour_fn);

        // Check if converged
        if copy == new_copy {
            break;
        }

        copy = new_copy;
    }

    return copy.iter().filter(|x| **x == '#').count();
}

pub fn a(input: &Array2<char>) -> usize {
    return converge(input, 4, neighbours);
}

pub fn b(input: &Array2<char>) -> usize {
    return converge(input, 5, line_of_sight_neighbours);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input().unwrap());
        assert_eq!(result, 2324);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input().unwrap());
        assert_eq!(result, 2068);
    }
}
