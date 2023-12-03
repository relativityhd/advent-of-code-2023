advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Split input into lines
    let numbers = input.lines().map(|line| {
        // Filter out non-numeric characters
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        // Convert digits to numbers
        let first = digits.next().expect("This to be a number");
        let number = match digits.last() {
            Some(num) => first * 10 + num,
            None => first * 10 + first,
        };
        number
    });
    Some(numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = input.lines().map(|line| {
        let mut digits = (0..line.len()).filter_map(|i| {
            let reduced_line = &line[i..];
            if reduced_line.starts_with("one") {
                return Some(1);
            } else if reduced_line.starts_with("two") {
                return Some(2);
            } else if reduced_line.starts_with("three") {
                return Some(3);
            } else if reduced_line.starts_with("four") {
                return Some(4);
            } else if reduced_line.starts_with("five") {
                return Some(5);
            } else if reduced_line.starts_with("six") {
                return Some(6);
            } else if reduced_line.starts_with("seven") {
                return Some(7);
            } else if reduced_line.starts_with("eight") {
                return Some(8);
            } else if reduced_line.starts_with("nine") {
                return Some(9);
            }
            reduced_line.chars().next().unwrap().to_digit(10)
        });
        // Convert digits to numbers
        let first = digits.next().expect("This to be a number");
        let number = match digits.last() {
            Some(num) => first * 10 + num,
            None => first * 10 + first,
        };
        number
    });

    Some(numbers.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
