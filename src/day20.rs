use countmap::CountMap;
use ndarray::s;
use ndarray::Array2;
use std::collections::HashMap;
use std::fs;

pub fn get_input() -> HashMap<usize, Array2<usize>> {
    let input = fs::read_to_string("input/day20.txt").expect("couldn't read input file");
    return input
        .split("\n\n")
        .map(|x| {
            let id = x.lines().nth(0).unwrap()[5..9].parse().unwrap();
            let tile = Array2::from_shape_vec(
                (x.lines().count() - 1, x.lines().nth(1).unwrap().len()),
                x.replace("#", "1")
                    .replace(".", "0")
                    .lines()
                    .skip(1)
                    .collect::<String>()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as usize)
                    .collect(),
            )
            .unwrap();

            (id, tile)
        })
        .collect();
}

pub fn a(input: &HashMap<usize, Array2<usize>>) -> usize {
    // General idea: find tiles with two edges without neighbours

    let mut edge_counter = CountMap::<usize, usize>::new();
    let slices = &[s![0, ..], s![-1, ..], s![.., 0], s![.., -1]];

    for (_, tile) in input {
        for slice in slices {
            let edge_hash = tile.slice(slice).fold(0, |acc, x| (acc << 1) + x);
            let reverse = tile
                .slice(slice)
                .slice(s![..;-1])
                .fold(0, |acc, x| (acc << 1) + x);

            edge_counter.insert_or_increment(edge_hash);
            edge_counter.insert_or_increment(reverse);
        }
    }

    return input
        .iter()
        .filter(|(_, tile)| {
            slices
                .iter()
                .map(|slice| tile.slice(slice).fold(0, |acc, x| (acc << 1) + x))
                .filter(|edge| edge_counter[&edge] == 1)
                .count()
                == 2
        })
        .map(|(id, _)| id)
        .product();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_a() {
        let result = a(&get_input());
        assert_eq!(result, 22878471088273);
    }
}
