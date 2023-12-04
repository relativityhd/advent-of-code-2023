advent_of_code::solution!(2);

use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| {
        let (id, data) = line.split_once(": ").unwrap();

        let mut is_possible = true;
        for pick in data.split("; ") {
            for color_tuple in pick.split(", ") {
                let (n, color) = color_tuple.split_once(' ').unwrap();
                let n: u32 = n.parse().unwrap();
                is_possible = match color {
                    "red" => is_possible && n <= 12,
                    "green" => is_possible && n <= 13,
                    "blue" => is_possible && n <= 14,
                    _ => unreachable!(),
                };
                if !is_possible {
                    return 0;
                }
            }
        }
        id.split_once(' ').unwrap().1.parse().unwrap()
    });
    Some(games.sum())
}

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| {
        let data = line.split_once(": ").unwrap().1;

        let game = data.split("; ").fold(
            Game {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut game, pick| {
                for color_tuple in pick.split(", ") {
                    let (n, color) = color_tuple.split_once(' ').unwrap();
                    let n: u32 = n.parse().unwrap();
                    if color == "red" {
                        game.red = max(game.red, n);
                    } else if color == "green" {
                        game.green = max(game.green, n);
                    } else if color == "blue" {
                        game.blue = max(game.blue, n);
                    }
                }
                game
            },
        );

        game.red * game.green * game.blue
    });
    Some(games.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
