use ndarray::Array2;
use std::fs;

pub fn get_input() -> Result<Array2<char>, ndarray::ShapeError> {
    let input = fs::read_to_string("input/day3.txt").expect("couldn't read input file");
    return Array2::from_shape_vec(
        (input.lines().count(), input.lines().nth(0).unwrap().len()),
        input.replace("\n", "").chars().collect(),
    );
}

pub fn a(input: Array2<char>, slope_x: usize, slope_y: usize) -> usize {
    let mut pos_x = 0;
    let mut pos_y = 0;

    let mut treecount = 0;

    loop {
        let tree = input[[pos_y, pos_x % input.shape()[1]]] == '#';
        treecount += tree as usize;

        pos_x += slope_x;
        pos_y += slope_y;

        if pos_y >= input.shape()[0] {
            return treecount;
        }
    }
}

pub fn b(input: Array2<char>) -> usize {
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;
    for slope in slopes {
        product *= a(input.clone(), slope.0, slope.1);
    }

    return product;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(get_input().unwrap(), 3, 1);
        assert_eq!(result, 184);
    }

    #[test]
    fn example_b() {
        let result = b(get_input().unwrap());
        assert_eq!(result, 2431272960);
    }
}
