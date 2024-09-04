advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn calculate(&self) -> u64 {
        let p = self.time as f64;
        let q = self.record as f64;
        let x1 = p / 2. + f64::sqrt(p * p / 4. - q);
        let x2 = p / 2. - f64::sqrt(p * p / 4. - q);
        let first = f64::min(x1, x2).floor() as u64 + 1;
        let second = f64::max(x1, x2).ceil() as u64 - 1;

        second - first + 1
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() != 2 {
        return races;
    }

    let time_values: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    let record_values: Vec<u64> = lines[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    for (time, record) in time_values.into_iter().zip(record_values.into_iter()) {
        races.push(Race { time, record });
    }

    races
}

fn parse_input_two(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();

    if lines.len() != 2 {
        panic!("Invalid input");
    }

    let time: u64 = lines[0]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();

    let record: u64 = lines[1]
        .split_whitespace()
        .skip(1)
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse()
        .unwrap();

    Race { time, record }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_input(input);
    Some(races.iter().map(|race| race.calculate()).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_input_two(input);
    Some(race.calculate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let races = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(races.len(), 3);
        assert_eq!(races[0], Race { time: 7, record: 9 });
        assert_eq!(
            races[1],
            Race {
                time: 15,
                record: 40
            }
        );
        assert_eq!(
            races[2],
            Race {
                time: 30,
                record: 200
            }
        );
    }

    #[test]
    fn test_parser_two() {
        let race = parse_input_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            race,
            Race {
                time: 71530,
                record: 940200
            }
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
