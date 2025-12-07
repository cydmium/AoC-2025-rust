use memoize::memoize;
use std::collections::HashSet;

advent_of_code::solution!(7);

struct Manifold {
    beams: HashSet<usize>,
    splitters: Vec<HashSet<usize>>,
}

fn parse(input: &str) -> Manifold {
    let mut lines = input.lines();
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(lines.next().unwrap().find(|c| c == 'S').unwrap());
    let splitters: Vec<HashSet<usize>> = lines
        .skip(1)
        .step_by(2)
        .map(|s| {
            s.char_indices()
                .filter_map(|(index, c)| if c == '^' { Some(index) } else { None })
                .collect()
        })
        .collect();
    Manifold { beams, splitters }
}

#[memoize(Ignore: splitters)]
fn count_timelines(row: usize, col: usize, splitters: &Vec<HashSet<usize>>) -> u64 {
    if row == splitters.len() {
        return 1;
    }
    if splitters[row].contains(&col) {
        return count_timelines(row + 1, col - 1, splitters)
            + count_timelines(row + 1, col + 1, splitters);
    }
    return count_timelines(row + 1, col, splitters);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut manifold = parse(input);
    let mut num_splits: u64 = 0;
    for row in manifold.splitters.iter() {
        for index in row.iter() {
            if manifold.beams.remove(index) {
                num_splits += 1;
                manifold.beams.insert(index - 1);
                manifold.beams.insert(index + 1);
            }
        }
    }
    Some(num_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut manifold = parse(input);
    let num_timelines = count_timelines(
        0,
        manifold.beams.drain().next().unwrap(),
        &manifold.splitters,
    );
    Some(num_timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
