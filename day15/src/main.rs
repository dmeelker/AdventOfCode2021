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

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<i32>>
}

impl Grid {
    fn new(data: Vec<Vec<i32>>) -> Grid {
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

    fn get(&self, point: &Point) -> Option<i32> {
        if self.contains(point) {
            Some(self.data[point.y as usize][point.x as usize])
        } else {
            None
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
    let input = fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(&input);
    grid.print();
    let part1 = part1(&grid);
    let part2 = part2(&grid);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Grid {
    Grid::new(input.lines().map(|line|
        line.chars().map(|c| c.to_string().parse().unwrap()).collect()
    ).collect())
}

fn part1(values: &Grid) -> i32 {
    let path = find_best_path(values, &Point {x: 0, y: 0} , &Point{ x: (values.width - 1) as i32, y: (values.height - 1) as i32});

    //eprintln!("path = {:?}", path);
    path.iter().skip(1).map(|p| values.get(p).unwrap()).sum()

}

fn part2(values: &Grid) -> i32 {
    let large_grid = prepare_part2_grid(values);
    //large_grid.print();
    

    let path = find_best_path(&large_grid, &Point {x: 0, y: 0} , &Point{ x: (large_grid.width - 1) as i32, y: (large_grid.height - 1) as i32});

    //eprintln!("path = {:?}", path);
    path.iter().skip(1).map(|p| large_grid.get(p).unwrap()).sum()
}

fn prepare_part2_grid(input: &Grid) -> Grid {
    let mut data = Vec::new();
    data.reserve(input.height * 5);

    for _ in 0..input.height * 5 {
        let mut column = Vec::new();
        column.resize(input.width*5, 0);
        data.push(column);
    }

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

    Grid::new(data)
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
        println!("Start!");
    let mut current_location = *start;
    let mut step = 1;
    loop {
        step+=1;
        // if step % 1000 == 0 {
        //     println!("Unvisited left: {}/{}", unvisited.len(), nodes.len());
        // }
        
        //eprintln!("current_location = {:?}", current_location);
        let neighbours = get_neighbours(&current_location);
        let neighbours = neighbours.iter().filter(|p| unvisited.contains(*p)).collect_vec();
        let current_node = nodes.get(&current_location).unwrap().clone();

        for neighbour in neighbours.iter() {
            let node = nodes.get_mut(*&neighbour).unwrap();
            //eprintln!("node.cost = {:?}", node.cost);
            let cost = current_node.path_cost + node.cost;
            //eprintln!("Neighbour = {:?} = {}", neighbour, cost);
            if cost < node.path_cost {
                node.path_cost = cost;
                node.path_source = Some(current_location);
                unvisited_queue.remove(&neighbour);
                unvisited_queue.push(**neighbour, i32::MAX - node.path_cost);
            }
        }

        unvisited.remove(&current_location);

        if current_location == *end {
            eprintln!("Path found!");
            break;
        }

        current_location = unvisited_queue.pop().unwrap().0;
        // current_location = *unvisited.iter().sorted_by(|a, b| 
        //     nodes.get(*a).unwrap().path_cost.cmp(&nodes.get(*b).unwrap().path_cost)).next().unwrap();
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

    // #[test]
    // fn part2_should_work() {
    //     let input = vec![String::from("123")];
    //     let result = part2(&input);

    //     assert_eq!(2, result);
    // }
}