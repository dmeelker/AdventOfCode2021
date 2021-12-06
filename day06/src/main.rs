use std::{fs, collections::HashMap};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input2.txt").unwrap();
    let values = parse_input(&input);

    let part1 = part1(&values);
    let part2 = part2(&values);

    println!("Part 1: {} Part 2: {}", part1, part2);
}

fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|p| p.parse().unwrap()).collect()
}

fn part1(values: &[i32]) -> usize {
    let mut fish = values.to_vec();

    for day in 0..80 {
        //println!("Day {}: {:?}", day, fish);
        for i in 0..fish.len() {
            let mut current_fish = fish[i];
            current_fish -= 1;

            if current_fish == -1 {
                current_fish = 6;
                fish.push(8);
            }

            fish[i] = current_fish;
        } 

        
    }

    fish.len()
}

fn part2(values: &[i32]) -> usize {
    let mut fish: HashMap<i32, usize> = HashMap::new();
    println!("{:?}", values);
    println!("{:?}", values.iter().counts_by(|v| v));
    
    for entry in values.iter().counts_by(|v| v).iter() {
        fish.insert(**entry.0, *entry.1);
    }

    for day in 0..18 {
        let mut new_fish: HashMap<i32, usize> = HashMap::new();
        println!("Day {}: {:?}", day, fish);
        for entry in fish.iter() {
            let mut age = *entry.0;
            let fish = *entry.1;
       
            age -= 1;

            if age == -1 {
                age = 6;
                //fish.push(8);

                new_fish.insert(8, new_fish.get(&8).unwrap_or(&0) + 1);
                //new_fish[&8] = new_fish.get(&8).unwrap_or(&0) + 1;
            }

            new_fish.insert(age, new_fish.get(&age).unwrap_or(&0) + fish);
            //new_fish[&age] = new_fish.get(&age).unwrap_or(&0) + fish;
        } 

        fish = new_fish;
    }

    fish.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_should_work() {
        let input = vec![String::from("123")];
        let result = part1(&input);

        assert_eq!(1, result);
    }

    #[test]
    fn part2_should_work() {
        let input = vec![String::from("123")];
        let result = part2(&input);

        assert_eq!(2, result);
    }
}