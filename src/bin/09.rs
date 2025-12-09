use core::fmt;
use std::cmp;

advent_of_code::solution!(9);

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use write! macro to write formatted output to the formatter
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rectangle {
    pt1: Point,
    pt2: Point,
}

impl Rectangle {
    fn area(&self) -> u64 {
        let dx = self.pt1.x.abs_diff(self.pt2.x) + 1;
        let dy = self.pt1.y.abs_diff(self.pt2.y) + 1;
        dx * dy
    }

    fn get_extremes(&self) -> (Point, Point) {
        let min_x = cmp::min(self.pt1.x, self.pt2.x);
        let min_y = cmp::min(self.pt1.y, self.pt2.y);
        let max_x = cmp::max(self.pt1.x, self.pt2.x);
        let max_y = cmp::max(self.pt1.y, self.pt2.y);

        (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
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
            }
        })
        .collect()
}

fn generate_rectangles(points: &Vec<Point>) -> Vec<Rectangle> {
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for i in 0..points.len() {
        let pt1 = &points[i];
        for j in i + 1..points.len() {
            let pt2 = &points[j];
            rectangles.push(Rectangle {
                pt1: *pt1,
                pt2: *pt2,
            });
        }
    }
    rectangles
}

fn valid_rectangle(rectangle: &Rectangle, borders: &Vec<(Point, Point)>) -> bool {
    borders
        .iter()
        .filter(|(pt1, pt2)| {
            let (bottom_left, top_right) = rectangle.get_extremes();
            let above_or_below = (pt1.y <= bottom_left.y && pt2.y <= bottom_left.y)
                || (pt1.y >= top_right.y && pt2.y >= top_right.y); // Border is fully above or below rectangle
            let left_or_right = (pt1.x <= bottom_left.x && pt2.x <= bottom_left.x)
                || (pt1.x >= top_right.x && pt2.x >= top_right.x); // Border is fully left or right of rectangle
            !(left_or_right || above_or_below) // Border is inside rectangle
        })
        .peekable()
        .peek()
        .is_none() // No borders inside rectangle
}

fn max_valid_area(rectangles: &Vec<Rectangle>, borders: &Vec<(Point, Point)>) -> Option<u64> {
    // Assume rectangles is sorted s.t. the largest area is at index 0
    for rectangle in rectangles.iter() {
        if valid_rectangle(rectangle, borders) {
            return Some(rectangle.area());
        }
    }
    None
}

fn generate_borders(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut borders: Vec<(Point, Point)> = Vec::new();
    for i in 1..points.len() {
        let pt1 = points[i - 1];
        let pt2 = points[i];
        borders.push((pt1, pt2));
    }
    borders.push((*points.last().unwrap(), points[0]));
    borders
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = parse(input);
    let mut rectangles: Vec<Rectangle> = generate_rectangles(&points);
    rectangles.sort_by(|a, b| b.area().cmp(&a.area()));
    Some(rectangles.first().unwrap().area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = parse(input);
    let mut rectangles: Vec<Rectangle> = generate_rectangles(&points);
    rectangles.sort_by(|a, b| b.area().cmp(&a.area()));
    let borders: Vec<(Point, Point)> = generate_borders(&points);

    max_valid_area(&rectangles, &borders)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
