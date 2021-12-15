use core::fmt;
use std::{fs, collections::{HashSet, HashMap}, slice::SliceIndex};
use itertools::Itertools;
use priority_queue::PriorityQueue;

//type Grid = Vec<Vec<i32>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<i32>>
}

impl Grid {
    fn new(width: usize, height: usize, default: i32) -> Grid {
        let mut data = Vec::new();

        for _ in 0..height {
            let mut column = Vec::new();
            column.resize(width, default);
            data.push(column);
        }

        Grid { width, height, data }
    }

    fn from_data(data: Vec<Vec<i32>>) -> Grid {
        let width = data[0].len();
        let height = data.len();

        Grid {
            width,
            height,
            data
        }
    }

    fn contains(&self, point: &Point) -> bool {
        !(point.x < 0 || point.x >= self.width as i32 || point.y < 0 || point.y >= self.height as i32)
    }

    fn set(&mut self, point: &Point, value: i32) {
        if self.contains(point) {
            self.data[point.y as usize][point.x as usize] = value;
        } else {
            panic!();
        }
    }

    fn get(&self, point: &Point) -> Option<i32> {
        if self.contains(point) {
            Some(self.data[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn copy_block(&self, target: &mut Grid, source_location: &Point, target_location: &Point, width: usize, height: usize) {
        for x in source_location.x..source_location.x + width as i32 {
            for y in source_location.y..source_location.y + height as i32 {
                let target_location  = Point::new(target_location.x + (x - source_location.x), target_location.y + (y - source_location.y));

                target.set(&target_location, self.get(&Point::new(x, y)).unwrap());
            }    
        }
    }

    fn increment_block(&mut self, location: &Point, width: usize, height: usize) {
        for x in location.x..location.x + width as i32 {
            for y in location.y..location.y + height as i32 {
                let location  = Point::new(x, y);
                let mut new_value = self.get(&location).unwrap() + 1;

                if new_value > 9 {
                    new_value = 1;
                }

                self.set(&location, new_value);
            }    
        }
    }

    fn get_all_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                points.push(Point {x: x as i32, y: y as i32});
            }    
        }

        points
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[y][x])
            }    

            println!();
        }
    }
}

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let grid = parse_input(&input);
    grid.print();
    let part1 = part1(&grid);
    let part2 = part2(&grid);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Grid {
    Grid::from_data(input.lines().map(|line|
        line.chars().map(|c| c.to_string().parse().unwrap()).collect()
    ).collect())
}

fn part1(grid: &Grid) -> i32 {
    let path = find_best_path(grid, &Point {x: 0, y: 0} , &Point{ x: (grid.width - 1) as i32, y: (grid.height - 1) as i32});

    path.iter().skip(1).map(|p| grid.get(p).unwrap()).sum()
}

fn part2(grid: &Grid) -> i32 {
    let grid = prepare_part2_grid(grid);
    let path = find_best_path(&grid, &Point {x: 0, y: 0} , &Point{ x: (grid.width - 1) as i32, y: (grid.height - 1) as i32});

    path.iter().skip(1).map(|p| grid.get(p).unwrap()).sum()
}

fn prepare_part2_grid(input: &Grid) -> Grid {
    let mut grid = Grid::new(input.width*5, input.height*5, 0);
    let mut data = Vec::new();

    for _ in 0..input.height * 5 {
        let mut column = Vec::new();
        column.resize(input.width*5, 0);
        data.push(column);
    }

    for y in 0..5 {
        for x in 0..5 {
            input.copy_block(&mut grid, &Point::new(0, 0), &Point::new(x * input.width as i32, y * input.height as i32), input.width, input.height);
            // let cost_increase = (x / input.width) as i32; 
            // data[y][x] = ((input.get(&Point {x: (x % input.width) as i32, y: y as i32}).unwrap() - 1 + cost_increase) % 9);
        }
    }

    

    grid.print();

    for y in 0..input.height {
        for x in 0..input.width * 5 {
            let cost_increase = (x / input.width) as i32; 
            data[y][x] = ((input.get(&Point {x: (x % input.width) as i32, y: y as i32}).unwrap() - 1 + cost_increase) % 9);
        }
    }

    for y in input.height..input.height*5 {
        for x in 0..input.width * 5 {
            let cost_increase = (y / input.height) as i32;
            data[y][x] = (data[y % input.height][x] + cost_increase) % 9;
        }
    }

    for y in 0..input.height*5 {
        for x in 0..input.width * 5 {
            data[y][x] += 1;
        }
    }

    Grid::from_data(data)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    location: Point,
    cost: i32,
    path_cost: i32,
    path_source: Option<Point>
}

fn find_best_path(grid: &Grid, start: &Point, end: &Point) -> Vec<Point> {
    let mut nodes: HashMap<Point, Node> = grid.get_all_points().iter().map(|p| 
        (
            *p,
            Node 
            {
                location: *p,
                cost: grid.get(p).unwrap(),
                path_cost: i32::MAX,
                path_source: None
            }
        )).collect();

    let mut unvisited: HashSet<Point> = nodes.keys().cloned().collect();
    let mut unvisited_queue: PriorityQueue<Point, i32> = PriorityQueue::new();
    
    for p in unvisited.iter() {
        unvisited_queue.push(*p, 0);
    }

    nodes.entry(*start).and_modify(|node| node.path_cost = 0);

    let mut current_location = *start;
    
    loop {
        let neighbours = get_neighbours(&current_location);
        let neighbours = neighbours.iter().filter(|p| unvisited.contains(*p)).collect_vec();
        let current_node = nodes.get(&current_location).unwrap().clone();

        for neighbour in neighbours.iter() {
            let neighbour_node = nodes.get_mut(*&neighbour).unwrap();
            let cost = current_node.path_cost + neighbour_node.cost;
            
            if cost < neighbour_node.path_cost {
                neighbour_node.path_cost = cost;
                neighbour_node.path_source = Some(current_location);
                unvisited_queue.push_increase(**neighbour, i32::MAX - neighbour_node.path_cost);
            }
        }

        unvisited.remove(&current_location);

        if current_location == *end {
            break;
        }

        current_location = unvisited_queue.pop().unwrap().0;
    }

    let mut path = Vec::new();
    let mut node = nodes.get(end).unwrap();

    while node.path_source.is_some() {
        path.push(node.location);
        node = match node.path_source {
            Some(previous) => nodes.get(&previous).unwrap(),
            _ => panic!()
        }
    }

    path.push(*start);

    path.iter().map(|p| *p).rev().collect()
}

fn get_neighbours(center: &Point) -> Vec<Point> {
    vec![
        Point { x: center.x - 0, y: center.y - 1},
        Point { x: center.x - 1, y: center.y},
        Point { x: center.x + 1, y: center.y},
        Point { x: center.x - 0, y: center.y + 1},
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_work() {
        let input = fs::read_to_string("input2.txt").unwrap();
        let input = parse_input(&input);
        let result = part1(&input);

        assert_eq!(40, result);
    }

    #[test]
    fn part2_should_work() {
        let input = fs::read_to_string("input2.txt").unwrap();
        let input = parse_input(&input);
        let result = part2(&input);

        assert_eq!(315, result);
    }
}