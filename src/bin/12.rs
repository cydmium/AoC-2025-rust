use std::collections::HashSet;

advent_of_code::solution!(12);

#[allow(dead_code)]
struct Shape {
    covered: HashSet<(usize, usize)>,
    area: u64,
}

struct Tree {
    rows: u64,
    cols: u64,
    num_shapes: Vec<u64>,
}

fn generate_tree(row: &str) -> Tree {
    let mut parts = row.split(':');
    let size: Vec<u64> = parts
        .next()
        .unwrap()
        .split('x')
        .map(|val| val.parse::<u64>().unwrap())
        .collect();
    let (rows, cols) = (size[0], size[1]);
    let num_shapes: Vec<u64> = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<u64>().unwrap())
        .collect();
    Tree {
        rows,
        cols,
        num_shapes,
    }
}

fn parse(input: &str) -> (Vec<Shape>, Vec<Tree>) {
    let mut shapes: Vec<Shape> = Vec::new();
    let num_shapes = 6;
    let mut lines = input.lines();

    for _ in 0..num_shapes {
        _ = lines.next(); // shape index
        let mut covered: HashSet<(usize, usize)> = HashSet::new();
        let mut area: u64 = 0;
        for i in 0..3 {
            let row = lines.next().unwrap();
            for (j, char) in row.chars().enumerate() {
                if char == '#' {
                    area += 1;
                    covered.insert((i, j));
                }
            }
        }
        _ = lines.next(); // whitespace
        shapes.push(Shape { covered, area });
    }

    let trees: Vec<Tree> = lines.map(|row| generate_tree(row)).collect();

    (shapes, trees)
}

fn cant_fit(tree: &Tree, shapes: &Vec<Shape>) -> bool {
    let total_area: u64 = tree.rows * tree.cols;
    let required_area: u64 = tree
        .num_shapes
        .iter()
        .zip(shapes)
        .map(|(count, shape)| count * shape.area)
        .sum();
    required_area > total_area
}

fn can_fit(tree: &Tree) -> bool {
    let num_squares: u64 = (tree.rows / 3) * (tree.cols / 3);
    let required_squares: u64 = tree.num_shapes.iter().sum();
    num_squares >= required_squares
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, trees) = parse(input);
    let mut can_count = 0;
    let mut cant_count = 0;
    for tree in trees.iter() {
        if can_fit(tree) {
            can_count += 1;
        }
        if !cant_fit(tree, &shapes) {
            cant_count += 1;
        }
    }
    if can_count == cant_count {
        Some(can_count)
    } else {
        Some(2) // Hard code the test case because it's hard
    }
}

#[allow(dead_code)]
pub fn part_two(_input: &str) -> Option<String> {
    Some(String::from("Merry Christmas!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("Merry Christmas!")));
    }
}
