advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Split input into lines
    let input: Vec<&str> = input.lines().collect();
    let numbers: Vec<u32> = input
        .into_iter()
        .map(|line| {
            // Filter out non-numeric characters
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            // Convert digits to numbers
            let number = digits[0] * 10 + digits[digits.len() - 1];
            number
        })
        .collect();
    Some(numbers.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let digit_names = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let lines: Vec<String> = input
        .lines()
        .map(|line| {
            let mut new_line = String::from("");
            let mut starting_char = 0;
            while starting_char < line.len() {
                let mut found_digit = false;
                for (digit, digit_name) in digit_names.iter().enumerate() {
                    if line[starting_char..].starts_with(digit_name) {
                        new_line.push_str(&digit.to_string());
                        starting_char += digit_name.len();
                        found_digit = true;
                        break;
                    }
                }
                if found_digit {
                    continue;
                }
                new_line.push(line.chars().nth(starting_char).unwrap());
                starting_char += 1;
            }
            new_line
        })
        .collect();
    let numbers: Vec<u32> = lines
        .into_iter()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let number = digits[0] * 10 + digits[digits.len() - 1];
            number
        })
        .collect();
    Some(numbers.iter().sum())
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
