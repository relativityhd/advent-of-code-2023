advent_of_code::solution!(4);

struct Card {
    winning_numbers: [u32; 10],
    scratch_numbers: [u32; 25],
}

impl Card {
    fn matches(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|winning_number| {
                **winning_number != 0 && self.scratch_numbers.contains(winning_number)
            })
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self.matches() as u32;
        match matches {
            0 => 0,
            _ => 2u32.pow(matches - 1),
        }
    }
}

fn get_card_from_line(line: &str) -> Card {
    let mut chars = line[5..].chars();
    let mut winning_numbers = [0; 10];
    let mut scratch_numbers = [0; 25];

    let mut last_char: char = ':';
    let mut recent_number = None;
    let mut i_winning_numbers = 0;
    let mut i_scratch_numbers = 0;

    for c in chars.by_ref() {
        if c == ':' {
            break;
        }
    }

    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).unwrap();
            if let Some(number) = recent_number {
                recent_number = None;
                winning_numbers[i_winning_numbers] = number * 10 + digit;
                i_winning_numbers += 1;
            } else {
                recent_number = Some(digit);
            }

            last_char = c;
            continue;
        } else if c == last_char {
            recent_number = Some(0);
            last_char = c;
            continue;
        }

        if c == '|' {
            last_char = c;
            break;
        }

        last_char = c;
    }

    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).unwrap();
            if let Some(number) = recent_number {
                recent_number = None;
                scratch_numbers[i_scratch_numbers] = number * 10 + digit;
                i_scratch_numbers += 1;
            } else {
                recent_number = Some(digit);
            }

            last_char = c;
            continue;
        } else if c == last_char {
            recent_number = Some(0);
        }
        last_char = c;
    }

    Card {
        winning_numbers,
        scratch_numbers,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| get_card_from_line(line).score())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(get_card_from_line);

    let mut copies = [0; 250];

    for (i, card) in cards.enumerate() {
        copies[i] += 1;
        let n_wins = card.matches();
        for j in 0..n_wins {
            copies[j + i + 1] += copies[i];
        }
    }
    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
