use std::collections::HashMap;

use rayon::prelude::*;

advent_of_code::solution!(8);

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

enum Direction {
    Left,
    Right,
}

struct Instructions {
    directions: Vec<Direction>,
}

impl From<&str> for Instructions {
    fn from(input: &str) -> Self {
        let directions = input
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect();

        Self { directions }
    }
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        let (name, links) = input.split_once(" = ").unwrap();
        let left = links[1..=3].to_string();
        let right = links[6..=8].to_string();

        Self {
            name: name.to_string(),
            left,
            right,
        }
    }
}

fn parse(input: &str) -> (Instructions, HashMap<String, Node>) {
    let mut lines = input.lines();
    let instructions = Instructions::from(lines.next().unwrap());
    lines.next();
    let nodes = lines
        .map(|line| {
            let node = Node::from(line);
            (node.name.clone(), node)
        })
        .collect();

    (instructions, nodes)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse(input);

    let mut iter = instructions.directions.iter().cycle();
    let mut current = String::from("AAA");
    let mut i = 0;
    loop {
        let direction = iter.next().unwrap();
        match direction {
            Direction::Left => current = nodes[&current].left.clone(),
            Direction::Right => current = nodes[&current].right.clone(),
        }
        i += 1;
        if current == "ZZZ" {
            break;
        }

        if i > 100000 {
            break;
        }
    }

    Some(i)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, nodes) = parse(input);
    let mut iter = instructions.directions.iter().cycle();
    let mut currents: Vec<String> = nodes
        .iter()
        .filter_map(|(name, _)| {
            if name.ends_with("A") {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();
    let mut distances = HashMap::new();
    let mut cycles = Vec::new();
    let mut i = 0;
    loop {
        let direction = iter.next().unwrap();
        currents = currents
            .par_iter()
            .map(|current| {
                let node = nodes.get(current).unwrap();
                match direction {
                    Direction::Left => node.left.clone(),
                    Direction::Right => node.right.clone(),
                }
            })
            .collect();
        i += 1;
        let mut currents_to_delete = Vec::new();
        for (pos, current) in currents.iter().enumerate() {
            if current.ends_with("Z") {
                if let Some(d) = distances.get(current) {
                    println!("Found cycle @{} for node {}: {}", i, current, i - d);
                    cycles.push((i - d) as u64);
                    currents_to_delete.push(pos);
                } else {
                    distances.insert(current.clone(), i);
                }
            }
        }
        for pos in currents_to_delete {
            currents.remove(pos);
        }

        if currents.len() == 0 || i > 100000 {
            break;
        }
    }
    println!("Cycles: {:?}", cycles);
    let mut ans = 1;
    for cycle in cycles.iter() {
        ans = lcm(ans, *cycle);
    }
    Some(ans)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
