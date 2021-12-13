use core::panic;
use std::{fs, collections::HashSet};
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(i32),
    Y(i32)
}

#[derive(Debug)]
struct Input {
    points: Vec<Point>,
    folds: Vec<Fold>
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(&input);

    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2:\n{}", part2);
}

fn parse_input(input: &str) -> Input {
    let parts = input.split("\r\n\r\n").collect_vec();

    Input { 
        points: parse_points(parts[0]),
        folds: parse_folds(parts[1]),
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().map(parse_point).collect_vec()
}

fn parse_point(input: &str) -> Point {
    let values: (&str, &str) = input.split(',').collect_tuple().unwrap();
    Point { 
        x: values.0.parse().unwrap(),
        y: values.1.parse().unwrap()
    }
}

fn parse_folds(input: &str) -> Vec<Fold> {
    input.lines().map(parse_fold).collect_vec()
}

fn parse_fold(input: &str) -> Fold {
    let values: (&str, &str) = input[11..].split('=').collect_tuple().unwrap();
    let value: i32 = values.1.parse().unwrap();

    match values.0 {
        "y" => Fold::Y(value),
        "x" => Fold::X(value),
        _ => panic!()
    }
}

fn fold(points: &HashSet<Point>, fold: Fold) -> HashSet<Point> {
    if let Fold::X(x) = fold {
        let left = points.iter().filter(|p| p.x < x).map(|p| *p);
        let right = points.iter().filter(|p| p.x > x).map(|p| Point{ x: x - (p.x - x), y: p.y});
        return left.chain(right).collect();
    } if let Fold::Y(y) = fold {
        let top = points.iter().filter(|p| p.y < y).map(|p| *p);
        let bottom = points.iter().filter(|p| p.y > y).map(|p| Point{ x: p.x, y: y - (p.y - y)});
        return top.chain(bottom).collect();
    } else {
        panic!();
    }
}

fn part1(input: &Input) -> usize {
    let mut points:  HashSet<Point> = input.points.iter().map(|p| *p).collect();
    points = fold(&points, input.folds[0]);
    points.len()
}

fn part2(input: &Input) -> String {
    let mut points:  HashSet<Point> = input.points.iter().map(|p| *p).collect();
    for f in input.folds.iter() {
        points = fold(&points, *f);
    }
    
    render_points(&points)
}

fn render_points(points: &HashSet<Point>) -> String {
    let mut chars: Vec<char> = Vec::new();

    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;

    for y in 0..height {
        for x in 0..width {
            let point = points.contains(&Point {x: x as i32, y: y as i32});

            if point {
                chars.push('#');
            } else {
                chars.push('.');
            }
        }

        chars.push('\n');
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_work() {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = parse_input(&input);
        let result = part1(&input);

        assert_eq!(664, result);
    }

    #[test]
    fn part2_should_work() {
        let input = fs::read_to_string("input.txt").unwrap();
        let input = parse_input(&input);
        let result = part2(&input);

        assert_eq!("####.####...##.#..#.####.#....###..#...
#....#.......#.#.#.....#.#....#..#.#...
###..###.....#.##.....#..#....###..#...
#....#.......#.#.#...#...#....#..#.#...
#....#....#..#.#.#..#....#....#..#.#...
####.#.....##..#..#.####.####.###..####
", result);
    }
}
