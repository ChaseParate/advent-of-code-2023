fn main() {
    let puzzle_input = include_str!("../puzzle_input.txt");

    println!("Part 1: {}", part_one(puzzle_input));
    println!("Part 2: {}", part_two(puzzle_input));
}

fn part_one(puzzle_input: &str) -> u32 {
    puzzle_input
        .lines()
        .map(|line| {
            let x = line.find(|c: char| c.is_ascii_digit()).unwrap();
            let x2 = line.rfind(|c: char| c.is_ascii_digit()).unwrap();

            let a: Vec<char> = line.chars().collect();
            let b = a[x].to_string();
            let b2 = a[x2].to_string();

            (b.parse::<u32>().unwrap() * 10) + b2.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_two(puzzle_input: &str) -> u32 {
    let nums = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    puzzle_input
        .lines()
        .map(|line| {
            let line_chars: Vec<char> = line.chars().collect();

            let word_stuff_left = nums
                .iter()
                .map(|(word, value)| (word, value, line.find(word)))
                .min_by_key(|(_, _, idx)| idx.unwrap_or(usize::MAX))
                .unwrap();
            let word_stuff_right = nums
                .iter()
                .map(|(word, value)| (word, value, line.rfind(word)))
                .max_by_key(|(_, _, idx)| idx.unwrap_or(usize::MIN))
                .unwrap();

            let left_most_digit = line.find(|c: char| c.is_ascii_digit());
            let right_most_digit = line.rfind(|c: char| c.is_ascii_digit());

            let first_digit = if word_stuff_left.2.unwrap_or(usize::MAX)
                < left_most_digit.unwrap_or(usize::MAX)
            {
                *word_stuff_left.1
            } else {
                line_chars
                    .get(left_most_digit.unwrap())
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap()
            };

            let second_digit = if word_stuff_right.2.unwrap_or(usize::MIN)
                > right_most_digit.unwrap_or(usize::MIN)
            {
                *word_stuff_right.1
            } else {
                line_chars
                    .get(right_most_digit.unwrap())
                    .unwrap()
                    .to_string()
                    .parse()
                    .unwrap()
            };

            (first_digit * 10) + second_digit
        })
        .sum()
}
