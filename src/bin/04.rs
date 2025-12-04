advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

struct ArrayIndex {
    row: usize,
    col: usize,
}

struct ArraySize {
    rows: usize,
    cols: usize,
}

impl ArraySize {
    fn contains(&self, row: isize, col: isize) -> bool {
        0 <= row && row < self.rows as isize && 0 <= col && col < self.cols as isize
    }
}

fn count_rolls(index: ArrayIndex, array_size: &ArraySize, array: &Vec<Vec<char>>) -> u64 {
    let array_ref = &array;
    let mut grid_mesh: Vec<(usize, usize)> = Vec::new();
    for row in (index.row as isize - 1)..=(index.row as isize + 1) {
        for col in (index.col as isize - 1)..=(index.col as isize + 1) {
            if array_size.contains(row, col) {
                grid_mesh.push((row.try_into().unwrap(), col.try_into().unwrap()));
            }
        }
    }

    grid_mesh
        .iter()
        .filter(|(row, col)| array_ref[*row][*col] == '@')
        .count() as u64
}

fn get_rolls(
    input: &Vec<Vec<char>>,
    grid_mesh: &Vec<(usize, usize)>,
    array_size: &ArraySize,
) -> Vec<(usize, usize)> {
    grid_mesh
        .iter()
        .filter(|(row, col)| {
            input[*row][*col] == '@'
                && count_rolls(
                    ArrayIndex {
                        row: *row,
                        col: *col,
                    },
                    array_size,
                    &input,
                ) < 5
        })
        .map(|(row, col)| (*row, *col))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let rows = input.len();
    let cols = input[0].len();
    let array_size = ArraySize { rows, cols };

    let mut grid_mesh: Vec<(usize, usize)> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            grid_mesh.push((row, col));
        }
    }
    Some(get_rolls(&input, &grid_mesh, &array_size).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse(input);
    let rows = input.len();
    let cols = input[0].len();
    let array_size = ArraySize { rows, cols };

    let mut grid_mesh: Vec<(usize, usize)> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            grid_mesh.push((row, col));
        }
    }

    let mut total_count: u64 = 0;
    loop {
        let removed_rolls = get_rolls(&input, &grid_mesh, &array_size);
        let num_removed = removed_rolls.len() as u64;
        if num_removed == 0 {
            break;
        }
        total_count += num_removed;
        removed_rolls
            .iter()
            .for_each(|(row, col)| input[*row][*col] = '.');
    }
    Some(total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
