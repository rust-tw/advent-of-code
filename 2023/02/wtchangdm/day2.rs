const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from(line: &str) -> Game {
        let split: Vec<&str> = line.split(": ").collect();
        let game_id = String::from(split[0]).replace("Game ", "");
        let mut game = Game {
            id: game_id.parse().unwrap(),
            rounds: vec![],
        };

        for round in split[1].split("; ").collect::<Vec<&str>>() {
            let mut r = Round {
                red: 0,
                green: 0,
                blue: 0,
            };
            let packages: Vec<&str> = round.split(", ").collect();

            for package in packages {
                let detail: Vec<&str> = package.split(' ').collect();

                let num: u32 = detail[0].parse().unwrap();
                let color = detail[1];

                match color {
                    "red" => r.red += num,
                    "green" => r.green += num,
                    "blue" => r.blue += num,
                    _ => panic!("how is this possible?"),
                }
            }

            game.rounds.push(r);
        }

        game
    }

    fn within_limits(&self) -> bool {
        self.rounds
            .iter()
            .all(|r| r.red <= RED_LIMIT && r.green <= GREEN_LIMIT && r.blue <= BLUE_LIMIT)
    }

    fn max_colors(&self) -> Round {
        self.rounds.iter().fold(
            Round {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, r| {
                acc.red = acc.red.max(r.red);
                acc.green = acc.green.max(r.green);
                acc.blue = acc.blue.max(r.blue);
                acc
            },
        )
    }
}

pub fn solve_part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| Game::from(line))
        .filter(|game| game.within_limits())
        .map(|game| game.id)
        .sum()
}

pub fn solve_part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| Game::from(line))
        .map(|game| game.max_colors())
        .map(|round| round.red * round.green * round.blue)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(solve_part2(&input), 2286);
    }
}
