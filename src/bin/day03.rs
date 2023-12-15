#[derive(Debug)]
struct SpatialNumber {
    number: u32,
    position: (usize, usize),
    length: usize,
}

fn main() {
    let puzzle_input = include_str!("../../day03_puzzle_input.txt");

    let (character_grid, spatial_numbers) = parse_puzzle_input(puzzle_input);

    println!("Part 1: {}", part_one(&character_grid, &spatial_numbers));
    println!("Part 2: {}", part_two(&character_grid, &spatial_numbers));
}

fn parse_puzzle_input(puzzle_input: &str) -> (Vec<Vec<char>>, Vec<SpatialNumber>) {
    let character_grid: Vec<Vec<char>> = puzzle_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut spatial_numbers = Vec::new();

    for (y, line) in character_grid.iter().enumerate() {
        let mut number = 0;
        let mut length = 0;

        for (x, char) in line.iter().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                number *= 10;
                number += digit;
                length += 1;
            } else if length > 0 {
                spatial_numbers.push(SpatialNumber {
                    number,
                    position: (x - length, y),
                    length,
                });

                number = 0;
                length = 0;
            }
        }

        if length > 0 {
            spatial_numbers.push(SpatialNumber {
                number,
                position: (line.len() - length, y),
                length,
            });
        }
    }

    (character_grid, spatial_numbers)
}

fn part_one(character_grid: &Vec<Vec<char>>, spatial_numbers: &Vec<SpatialNumber>) -> u32 {
    let mut sum = 0;

    'number: for SpatialNumber {
        number,
        position: (start_x, start_y),
        length,
    } in spatial_numbers
    {
        let start_x = *start_x as i32;
        let start_y = *start_y as i32;

        for y in start_y - 1..=start_y + 1 {
            if y < 0 || y >= character_grid.len() as i32 {
                continue;
            }

            for x in start_x - 1..=start_x + (*length as i32) {
                if x < 0 || x >= character_grid[y as usize].len() as i32 {
                    continue;
                }

                let char = character_grid[y as usize][x as usize];
                if !char.is_ascii_digit() && char != '.' {
                    sum += number;
                    continue 'number;
                }
            }
        }
    }

    sum
}

fn part_two(character_grid: &Vec<Vec<char>>, spatial_numbers: &Vec<SpatialNumber>) -> u32 {
    let mut summed_gear_ratios = 0;

    for (star_y, row) in character_grid.iter().enumerate() {
        for (star_x, char) in row.iter().enumerate() {
            let star_x = star_x as i32;
            let star_y = star_y as i32;

            if *char != '*' {
                continue;
            }

            let mut adjacent_part_numbers = Vec::new();

            for SpatialNumber {
                number,
                position: (number_x, number_y),
                length,
            } in spatial_numbers
            {
                let number_x = *number_x as i32;
                let number_y = *number_y as i32;

                if star_y + 1 >= number_y
                    && star_y - 1 <= number_y
                    && star_x + 1 >= number_x
                    && star_x <= number_x + (*length as i32)
                {
                    adjacent_part_numbers.push(number);
                    if adjacent_part_numbers.len() == 2 {
                        summed_gear_ratios += adjacent_part_numbers.into_iter().product::<u32>();
                        break;
                    }
                }
            }
        }
    }

    summed_gear_ratios
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let sample_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (character_grid, spatial_numbers) = parse_puzzle_input(sample_input);

        assert_eq!(part_one(&character_grid, &spatial_numbers), 4361);
    }

    #[test]
    fn part_two_example() {
        let sample_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let (character_grid, spatial_numbers) = parse_puzzle_input(sample_input);

        assert_eq!(part_two(&character_grid, &spatial_numbers), 467835);
    }
}
