use ndarray::Array2;
use std::collections::HashSet;
use std::fs;

type Coord = Vec<isize>;

pub trait NeighbourExt {
    fn neighbours(&self) -> Vec<Coord>;
}

impl NeighbourExt for Vec<isize> {
    fn neighbours(&self) -> Vec<Coord> {
        let mut vec = Vec::<Coord>::new();
        let dim = self.len();

        if dim == 0 {
            return vec![];
        }

        for x in -1..2 {
            let n = vec![self[0] + x];

            if dim == 1 {
                vec.push(n.clone());
            }

            for neigh in self[1..].to_vec().neighbours() {
                vec.push([n.clone(), neigh].concat());
            }
        }

        return vec;
    }
}

pub fn get_input() -> Result<Array2<usize>, ndarray::ShapeError> {
    let input = fs::read_to_string("input/day17.txt").expect("couldn't read input file");
    return Array2::from_shape_vec(
        (input.lines().count(), input.lines().nth(0).unwrap().len()),
        input
            .replace("\n", "")
            .replace("#", "1")
            .replace(".", "0")
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect(),
    );
}

pub fn a(input: &Array2<usize>, dims: usize) -> usize {
    let base_vec = vec![0; dims - 2];
    let mut active_cubes = input
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == 1)
        .map(|(coord, _)| {
            [
                vec![
                    coord as isize / input.shape()[1] as isize,
                    coord as isize % input.shape()[1] as isize,
                ],
                base_vec.clone(),
            ]
            .concat()
        })
        .collect::<HashSet<Coord>>();

    for _ in 0..6 {
        let mut next_active_cubes = HashSet::<Coord>::new();

        for active in active_cubes.iter() {
            let neighbour_cubes = active.neighbours();
            let mut neighbour_cubes_active = 0;

            for neighbour_cube in neighbour_cubes {
                if neighbour_cube == *active {
                    continue;
                }

                let neighbour_is_active = active_cubes.contains(&neighbour_cube);

                if neighbour_is_active {
                    neighbour_cubes_active += 1;
                }

                let neighbour_neighbour_cubes = neighbour_cube.neighbours();
                let active_neighbour_neighbours = neighbour_neighbour_cubes
                    .iter()
                    .filter(|x| active_cubes.contains(*x))
                    .count();

                if !neighbour_is_active && active_neighbour_neighbours == 3 {
                    next_active_cubes.insert(neighbour_cube);
                }
            }

            if neighbour_cubes_active == 2 || neighbour_cubes_active == 3 {
                next_active_cubes.insert(active.clone());
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
        let result = a(&get_input().unwrap(), 3);
        assert_eq!(result, 317);
    }

    #[test]
    fn example_b() {
        let result = a(&get_input().unwrap(), 4);
        assert_eq!(result, 1692);
    }
}
