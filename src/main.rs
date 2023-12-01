use std::collections::HashMap;

fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");

    println!("Part 1: {}", part_one(puzzle_input));
    println!("Part 2: {}", part_two(puzzle_input));
}

fn part_one(puzzle_input: &str) -> u32 {
    puzzle_input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let first_digit = chars
                .iter()
                .find(|c| c.is_ascii_digit())
                .and_then(|n| n.to_digit(10))
                .unwrap();

            let second_digit = chars
                .iter()
                .rfind(|c| c.is_ascii_digit())
                .and_then(|n| n.to_digit(10))
                .unwrap();

            (first_digit * 10) + second_digit
        })
        .sum()
}

fn part_two(puzzle_input: &str) -> u32 {
    let word_to_number_map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();

    puzzle_input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let (left_word_index, left_word_value) = word_to_number_map
                .iter()
                .map(|(word, value)| (line.find(word), value))
                .min_by_key(|(index, _)| index.unwrap_or(usize::MAX))
                .unwrap();
            let (right_word_index, right_word_value) = word_to_number_map
                .iter()
                .map(|(word, value)| (line.rfind(word), value))
                .max_by_key(|(index, _)| index.unwrap_or(usize::MIN))
                .unwrap();

            let left_digit_index = line.find(|c: char| c.is_ascii_digit());
            let right_digit_index = line.rfind(|c: char| c.is_ascii_digit());

            let first_digit =
                if left_word_index.unwrap_or(usize::MAX) < left_digit_index.unwrap_or(usize::MAX) {
                    *left_word_value
                } else {
                    chars[left_digit_index.unwrap()].to_digit(10).unwrap()
                };

            let second_digit = if right_word_index.unwrap_or(usize::MIN)
                > right_digit_index.unwrap_or(usize::MIN)
            {
                *right_word_value
            } else {
                chars[right_digit_index.unwrap()].to_digit(10).unwrap()
            };

            (first_digit * 10) + second_digit
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let sample_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part_one(sample_input), 142);
    }

    #[test]
    fn part_two_example() {
        let sample_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part_two(sample_input), 281);
    }
}
