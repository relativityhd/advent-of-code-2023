advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|measurement| measurement.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn create_walks(history: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut walks: Vec<Vec<i64>> = Vec::new();
    walks.push(history.clone());
    loop {
        let new_walk: Vec<i64> = walks
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        let first_val = new_walk.first().unwrap();
        let found = new_walk.iter().all(|x| x == first_val) || new_walk.len() == 1;
        walks.push(new_walk);
        if found {
            break;
        }
    }
    walks
}

pub fn part_one(input: &str) -> Option<i64> {
    let hists = parse(input);

    let extrapolations = hists.iter().map(|history| {
        let walks = create_walks(&history);
        let mut current_ext = 0;
        for walk in walks.iter().rev() {
            current_ext = walk.last().unwrap() + current_ext;
        }
        current_ext
    });

    Some(extrapolations.sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let hists = parse(input);

    let extrapolations = hists.iter().map(|history| {
        let walks = create_walks(&history);
        let mut current_ext = 0;
        for walk in walks.iter().rev() {
            current_ext = walk.first().unwrap() - current_ext;
        }
        current_ext
    });

    Some(extrapolations.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
