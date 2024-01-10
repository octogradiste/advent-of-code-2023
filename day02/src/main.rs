use std::fs::read_to_string;

const RED_CUBES: i32 = 12;
const GREEN_CUBES: i32 = 13;
const BLUE_CUBES: i32 = 14;

const FILE: &str = "data/input";

struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

const ROUND_ZERO: Round = Round {
    red: 0,
    green: 0,
    blue: 0,
};

impl Round {
    fn max(&self, other: &Round) -> Round {
        Round {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_valid(&self) -> bool {
        self.red <= RED_CUBES && self.green <= GREEN_CUBES && self.blue <= BLUE_CUBES
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: i32,
    rounds: Vec<Round>,
}

fn to_game(line: &str) -> Game {
    let mut parts = line.split(":");
    let game_id = parts.next().expect("Invalid input, no game id");
    let rounds = parts.next().expect("Invalid input, no rounds");

    let id: i32 = game_id[5..]
        .parse()
        .expect("Invalid input, game id is not a number");

    let mut game = Game { id, rounds: vec![] };

    let rounds = rounds.split(";");
    for round in rounds {
        let mut r = ROUND_ZERO;
        let colors = round.split(",");
        for color in colors {
            let mut parts = color.trim().split(" ");
            let count = parts.next().expect("Invalid input, no count");
            let name = parts.next().expect("Invalid input, no color name");
            match name {
                "red" => r.red = count.parse().expect("Invalid input, count is not a number"),
                "green" => r.green = count.parse().expect("Invalid input, count is not a number"),
                "blue" => r.blue = count.parse().expect("Invalid input, count is not a number"),
                _ => panic!("Invalid input, unknown color"),
            }
        }
        game.rounds.push(r);
    }

    game
}

fn solve_part1(game: Game) -> i32 {
    for round in game.rounds {
        if !round.is_valid() {
            return 0;
        }
    }
    return game.id;
}

fn solve_part2(game: Game) -> i32 {
    let mut max = ROUND_ZERO;
    for round in game.rounds {
        max = max.max(&round);
    }
    return max.power();
}

fn main() {
    let file = read_to_string(FILE).unwrap();
    let games = file.lines().map(to_game);

    let sum1: i32 = games.clone().map(solve_part1).sum();
    let sum2: i32 = games.clone().map(solve_part2).sum();

    println!("part1: {}", sum1);
    println!("part2: {}", sum2);
}
