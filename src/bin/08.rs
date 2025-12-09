use core::fmt;
use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, pt2: &Point) -> f64 {
        let dx = self.x - pt2.x;
        let dy = self.y - pt2.y;
        let dz = self.z - pt2.z;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! macro to write formatted output to the formatter
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|s| {
            let mut parts = s.split(",").into_iter();
            Point {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn compute_distances(points: &Vec<Point>) -> Vec<Vec<f64>> {
    let mut distances: Vec<Vec<f64>> = Vec::new();
    for i in 0..points.len() {
        let mut row: Vec<f64> = Vec::new();
        for j in 0..points.len() {
            row.push(points[i].distance(&points[j]));
        }
        distances.push(row);
    }
    distances
}

fn compute_circuits_pt1(distances: Vec<Vec<f64>>, num_connections: usize) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let mut connections: Vec<(usize, usize)> = Vec::new();
    for i in 0..distances.len() {
        for j in i + 1..distances.len() {
            connections.push((i, j));
        }
    }
    connections.sort_by(|a, b| distances[a.0][a.1].total_cmp(&distances[b.0][b.1]));

    for i in 0..num_connections {
        let mut inserted = false;
        let mut inserted_ind: usize = 0;
        let (pt1, pt2) = connections[i];
        let mut j: usize = 0;
        while j < circuits.len() {
            if circuits[j].contains(&pt1) || circuits[j].contains(&pt2) {
                if inserted {
                    let circuit = circuits.remove(j);
                    circuits[inserted_ind].extend(circuit.iter().cloned());
                    break;
                }
                circuits[j].insert(pt1);
                circuits[j].insert(pt2);
                inserted = true;
                inserted_ind = j;
            }
            j += 1;
        }
        if !inserted {
            circuits.push(HashSet::from([pt1, pt2]));
        }
    }

    circuits
}

fn compute_circuits_pt2(distances: Vec<Vec<f64>>) -> (usize, usize) {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let mut connections: Vec<(usize, usize)> = Vec::new();
    for i in 0..distances.len() {
        for j in i + 1..distances.len() {
            connections.push((i, j));
        }
    }
    connections.sort_by(|a, b| distances[a.0][a.1].total_cmp(&distances[b.0][b.1]));

    let mut i = 0;
    let mut result: (usize, usize) = (0, 0);
    while circuits.len() == 0 || !(circuits.len() == 1 && circuits[0].len() == distances.len()) {
        let mut inserted = false;
        let mut inserted_ind: usize = 0;
        let (pt1, pt2) = connections[i];
        let mut j: usize = 0;
        while j < circuits.len() {
            if circuits[j].contains(&pt1) || circuits[j].contains(&pt2) {
                if inserted {
                    let circuit = circuits.remove(j);
                    circuits[inserted_ind].extend(circuit.iter().cloned());
                    break;
                }
                circuits[j].insert(pt1);
                circuits[j].insert(pt2);
                inserted = true;
                inserted_ind = j;
            }
            j += 1;
        }
        if !inserted {
            circuits.push(HashSet::from([pt1, pt2]));
        }
        i += 1;
        result = (pt1, pt2);
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    // let num_connections = 10; // Test case
    let num_connections = 1000; // Real input
    let distances = compute_distances(&points);
    let mut circuits = compute_circuits_pt1(distances, num_connections);
    circuits.sort_by(|a, b| a.len().cmp(&b.len()));
    Some(
        circuits
            .iter()
            .rev()
            .take(3)
            .map(|circuit| circuit.len() as u64)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);

    let distances = compute_distances(&points);
    let indices = compute_circuits_pt2(distances);
    let pt1 = &points[indices.0];
    let pt2 = &points[indices.1];

    Some((pt1.x * pt2.x).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
