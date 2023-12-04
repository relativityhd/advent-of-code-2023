advent_of_code::solution!(3);

struct SymbolPosition {
    c: char,
    row: usize,
    col: usize,
}

struct NumberPosition {
    n: u32,
    row: usize,
    col_start: usize,
    col_end: usize,
}

fn parse_input(input: &str) -> (Vec<SymbolPosition>, Vec<NumberPosition>) {
    let mut symbols: Vec<SymbolPosition> = vec![];
    let mut numbers: Vec<NumberPosition> = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut col_start: Option<usize> = None;

        for (j, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() {
                if let Some(k) = col_start {
                    numbers.push(NumberPosition {
                        n: line[k..j].parse().unwrap(),
                        row: i,
                        col_start: k,
                        col_end: j - 1,
                    });
                    col_start = None;
                }
            }
            if c == '.' {
                continue;
            }
            if c.is_ascii_digit() {
                if col_start.is_none() {
                    col_start = Some(j);
                }
                continue;
            }
            symbols.push(SymbolPosition { c, row: i, col: j });
        }

        if let Some(k) = col_start {
            numbers.push(NumberPosition {
                n: line[k..].parse().unwrap(),
                row: i,
                col_start: k,
                col_end: line.len() - 1,
            });
        }
    }

    (symbols, numbers)
}

fn is_adjected(n: &NumberPosition, s: &SymbolPosition) -> bool {
    s.row.abs_diff(n.row) <= 1 && n.col_start <= s.col + 1 && s.col <= n.col_end + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse_input(input);

    let numbers = numbers.iter().filter_map(|n| {
        let valid = symbols.iter().any(|s| is_adjected(n, s));
        if valid {
            Some(n.n)
        } else {
            None
        }
    });

    Some(numbers.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse_input(input);

    let gear_ratios = symbols.iter().filter_map(|s| {
        if s.c != '*' {
            return None;
        }
        let adj_numbers: Vec<u32> = numbers
            .iter()
            .filter_map(|n| if is_adjected(n, s) { Some(n.n) } else { None })
            .collect();
        if adj_numbers.len() < 2 {
            return None;
        }
        Some(adj_numbers.iter().product::<u32>())
    });
    Some(gear_ratios.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
