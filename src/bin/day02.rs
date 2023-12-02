fn main() {
    let puzzle_input = include_str!("../../day02_puzzle_input.txt");

    println!("Part 1: {}", part_one(puzzle_input));
    println!("Part 2: {}", part_two(puzzle_input));
}

fn part_one(puzzle_input: &str) -> u32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    let x: Vec<_> = puzzle_input
        .lines()
        .map(|line| {
            let (game_id, rules) = line.split_once(':').unwrap();
            let (_, y) = game_id.split_once(' ').unwrap();
            let game_id_num: u32 = y.parse().unwrap();
            let rules_list: Vec<_> = rules
                .split(';')
                .map(|round| {
                    round
                        .trim()
                        .split(',')
                        .map(|thing| {
                            let (x, y) = thing.trim().split_once(' ').unwrap();
                            let num: u32 = x.parse().unwrap();
                            (num, y)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            (game_id_num, rules_list)
        })
        .collect();

    // println!("{:?}", &x[4]);

    // todo!();

    let mut sum = 0;
    for (game_id, rounds) in &x {
        let mut has_failed = false;

        for round in rounds {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for (num, pull_color) in round {
                match *pull_color {
                    "red" => red_count += num,
                    "green" => green_count += num,
                    "blue" => blue_count += num,
                    _ => {
                        panic!("invalid color {}", pull_color);
                    }
                }
            }

            if red_count > red || green_count > green || blue_count > blue {
                // dbg!(game_id, red_count, green_count, blue_count);
                has_failed = true;
            }
        }

        if !has_failed {
            sum += game_id;
        }
    }

    sum
}

fn part_two(puzzle_input: &str) -> u32 {
    let x: Vec<_> = puzzle_input
        .lines()
        .map(|line| {
            let (game_id, rules) = line.split_once(':').unwrap();
            let (_, y) = game_id.split_once(' ').unwrap();
            let game_id_num: u32 = y.parse().unwrap();
            let rules_list: Vec<_> = rules
                .split(';')
                .map(|round| {
                    round
                        .trim()
                        .split(',')
                        .map(|thing| {
                            let (x, y) = thing.trim().split_once(' ').unwrap();
                            let num: u32 = x.parse().unwrap();
                            (num, y)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            (game_id_num, rules_list)
        })
        .collect();

    let mut sum = 0;
    for (game_id, rounds) in &x {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;

        for round in rounds {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for (num, pull_color) in round {
                match *pull_color {
                    "red" => red_count += num,
                    "green" => green_count += num,
                    "blue" => blue_count += num,
                    _ => {
                        panic!("invalid color {}", pull_color);
                    }
                }
            }

            min_reds = min_reds.max(red_count);
            min_greens = min_greens.max(green_count);
            min_blues = min_blues.max(blue_count);
        }

        dbg!(min_reds, min_greens, min_blues);
        let power = min_reds * min_greens * min_blues;
        sum += power;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_one(sample_input), 8);
    }

    #[test]
    fn part_two_example() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

                assert_eq!(part_two(sample_input), 2286);
    }
}
