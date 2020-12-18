use ndarray::Array2;
use std::collections::HashSet;
use std::fs;

type Coord3 = (isize, isize, isize);
type Coord4 = (isize, isize, isize, isize);

pub fn get_input() -> Result<Array2<usize>, ndarray::ShapeError> {
    let input = fs::read_to_string("input/day17.txt").expect("couldn't read input file");
    return Array2::from_shape_vec(
        (
            input.lines().count(),
            input.lines().nth(0).unwrap().len()
        ),
        input
            .replace("\n", "")
            .replace("#", "1")
            .replace(".", "0")
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect(),
    );
}

pub fn neighbours3(coord: &Coord3) -> Vec<Coord3> {
    let mut vec = Vec::<Coord3>::new();

    for x in coord.0 - 1..coord.0 + 2 {
        for y in coord.1 - 1..coord.1 + 2 {
            for z in coord.2 - 1..coord.2 + 2 {
                if x == coord.0 && y == coord.1 && z == coord.2 {
                    continue;
                }
                vec.push((x, y, z));
            }
        }
    }

    return vec;
}

pub fn neighbours4(coord: &Coord4) -> Vec<Coord4> {
    let mut vec = Vec::<Coord4>::new();

    for x in coord.0 - 1..coord.0 + 2 {
        for y in coord.1 - 1..coord.1 + 2 {
            for z in coord.2 - 1..coord.2 + 2 {
                for w in coord.3 - 1..coord.3 + 2 {
                    if x == coord.0 && y == coord.1 && z == coord.2 && w == coord.3 {
                        continue;
                    }
                    vec.push((x, y, z, w));
                }
            }
        }
    }

    return vec;
}

pub fn a(input: &Array2<usize>) -> usize {
    let mut active_cubes = input
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == 1)
        .map(|(coord, _)| {
            (
                coord as isize / input.shape()[1] as isize,
                coord as isize % input.shape()[1] as isize,
                0,
            )
        })
        .collect::<HashSet<Coord3>>();

    for _ in 0..6 {
        let mut next_active_cubes = HashSet::<Coord3>::new();

        for active in active_cubes.iter() {
            let neighbour_cubes = neighbours3(active);
            let mut neighbour_cubes_active = 0;

            for neighbour_cube in neighbour_cubes {
                let neighbour_is_active = active_cubes.contains(&neighbour_cube);

                if neighbour_is_active {
                    neighbour_cubes_active += 1;
                }

                let neighbour_neighbour_cubes = neighbours3(&neighbour_cube);
                let active_neighbour_neighbours = neighbour_neighbour_cubes
                    .iter()
                    .filter(|x| active_cubes.contains(x))
                    .count();

                if !neighbour_is_active && active_neighbour_neighbours == 3 {
                    next_active_cubes.insert(neighbour_cube);
                }
            }

            if neighbour_cubes_active == 2 || neighbour_cubes_active == 3 {
                next_active_cubes.insert(*active);
            }
        }
        active_cubes = next_active_cubes;
    }

    return active_cubes.len();
}

// Rust doesn't support generic tuples, so copy paste it is..
// TODO: create a generic wrapper type that implements a `neighbours` trait
pub fn b(input: &Array2<usize>) -> usize {
    let mut active_cubes = input
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == 1)
        .map(|(coord, _)| {
            (
                coord as isize / input.shape()[1] as isize,
                coord as isize % input.shape()[1] as isize,
                0,
                0,
            )
        })
        .collect::<HashSet<Coord4>>();

    for _ in 0..6 {
        let mut next_active_cubes = HashSet::<Coord4>::new();

        for active in active_cubes.iter() {
            let neighbour_cubes = neighbours4(active);
            let mut neighbour_cubes_active = 0;

            for neighbour_cube in neighbour_cubes {
                let neighbour_is_active = active_cubes.contains(&neighbour_cube);

                if neighbour_is_active {
                    neighbour_cubes_active += 1;
                }

                let neighbour_neighbour_cubes = neighbours4(&neighbour_cube);
                let active_neighbour_neighbours = neighbour_neighbour_cubes
                    .iter()
                    .filter(|x| active_cubes.contains(x))
                    .count();

                if !neighbour_is_active && active_neighbour_neighbours == 3 {
                    next_active_cubes.insert(neighbour_cube);
                }
            }

            if neighbour_cubes_active == 2 || neighbour_cubes_active == 3 {
                next_active_cubes.insert(*active);
            }
        }
        active_cubes = next_active_cubes;
    }

    return active_cubes.len();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input().unwrap());
        assert_eq!(result, 317);
    }

    #[test]
    fn example_b() {
        let result = b(&get_input().unwrap());
        assert_eq!(result, 1692);
    }
}
