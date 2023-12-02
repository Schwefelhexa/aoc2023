use anyhow::Result;
use std::cmp::max;

pub fn run_day(data: &str) -> Result<()> {
    let max_game = Game {
        id: 0,
        max_red: 12,
        max_green: 13,
        max_blue: 14
    };

    let games = data.lines().map(|l| parse_line(l)).collect::<Vec<_>>();

    let possible_games = games.iter().filter(|g| g.possible(&max_game));
    let sum: u32 = possible_games.map(|g| g.id).sum();
    println!("{sum}");

    let sum_part_2: u32 = games.iter().map(|g| g.power()).sum();
    println!("{sum_part_2}");

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn possible(&self, max_game: &Game) -> bool {
        self.max_red <= max_game.max_red &&
        self.max_green <= max_game.max_green &&
        self.max_blue <= max_game.max_blue 
    }

    fn power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

fn parse_line(line: &str) -> Game {
    let mut result = Game {
        id: 0,
        max_red: 0,
        max_green: 0,
        max_blue: 0,
    };

    let (game, line) = line.split_once(":").expect("Line didn't contain a ':'");
    let (_, game_id) = game.split_once(" ").expect("Game didn't contain a space");
    let game_id = game_id.parse().expect("Failed to parse game_id");
    result.id = game_id;

    let draws = line.split(";").map(|d| d.trim());
    for draw in draws {
        let colors = draw.split(",").map(|c| c.trim());
        for color in colors {
            let (amount, name) = color.split_once(" ").expect("Color didn't contain a space");
            let amount = amount.parse().expect("Failed to parse amount");

            match name {
                "red" => result.max_red = max(result.max_red, amount),
                "green" => result.max_green = max(result.max_green, amount),
                "blue" => result.max_blue = max(result.max_blue, amount),
                _ => panic!("Unkown color name '{name}'"),
            }
        }
    }

    result
}
