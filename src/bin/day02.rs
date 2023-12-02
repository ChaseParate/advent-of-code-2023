enum Color {
    Red,
    Green,
    Blue,
}

struct Pull {
    color: Color,
    count: u32,
}

type Round = Vec<Pull>;
type Game = (u32, Vec<Round>);

fn main() {
    let puzzle_input = include_str!("../../day02_puzzle_input.txt");

    let games: Vec<Game> = parse_puzzle_input(puzzle_input);

    println!("Part 1: {}", part_one(&games));
    println!("Part 2: {}", part_two(&games));
}

fn parse_puzzle_input(puzzle_input: &str) -> Vec<Game> {
    puzzle_input
        .lines()
        .map(|line| {
            let (game_heading, rounds) = line.split_once(": ").unwrap();

            let game_id = game_heading.split_once(' ').unwrap().1.parse().unwrap();

            let rounds: Vec<_> = rounds
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        .map(|pull| {
                            let (count, color) = pull.split_once(' ').unwrap();

                            let count: u32 = count.parse().unwrap();
                            let color = match color {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                _ => unreachable!(),
                            };

                            Pull { color, count }
                        })
                        .collect()
                })
                .collect();

            (game_id, rounds)
        })
        .collect()
}

fn part_one(games: &[Game]) -> u32 {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    let mut sum = 0;
    for (game_id, rounds) in games {
        let mut has_failed = false;

        for round in rounds {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for Pull { color, count } in round {
                match color {
                    Color::Red => red_count += count,
                    Color::Green => green_count += count,
                    Color::Blue => blue_count += count,
                }
            }

            if red_count > RED_LIMIT || green_count > GREEN_LIMIT || blue_count > BLUE_LIMIT {
                has_failed = true;
                break;
            }
        }

        if !has_failed {
            sum += game_id;
        }
    }

    sum
}

fn part_two(games: &[Game]) -> u32 {
    let mut sum = 0;
    for (_game_id, rounds) in games {
        let mut min_reds = 0;
        let mut min_greens = 0;
        let mut min_blues = 0;

        for round in rounds {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;

            for Pull { color, count } in round {
                match *color {
                    Color::Red => red_count += count,
                    Color::Green => green_count += count,
                    Color::Blue => blue_count += count,
                }
            }

            min_reds = min_reds.max(red_count);
            min_greens = min_greens.max(green_count);
            min_blues = min_blues.max(blue_count);
        }

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
        let games = parse_puzzle_input(sample_input);

        assert_eq!(part_one(&games), 8);
    }

    #[test]
    fn part_two_example() {
        let sample_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_puzzle_input(sample_input);

        assert_eq!(part_two(&games), 2286);
    }
}
