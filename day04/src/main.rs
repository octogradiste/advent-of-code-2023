use std::{collections::HashSet, fs::read_to_string};

const FILE: &str = "data/input";
// const FILE: &str = "data/example1";

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn from(line: &str) -> Result<Card, &str> {
        let mut parts = line.split(":");
        parts.next(); // Skip card id
        let values = parts.next().ok_or("No values")?;

        let mut parts = values.split("|");
        let winning = parts.next().ok_or("No winning numbers")?;
        let numbers = parts.next().ok_or("No numbers")?;

        let winning = winning
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().ok())
            .collect::<Option<HashSet<u32>>>()
            .ok_or("Failed to parse the winning numbers")?;

        let numbers = numbers
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().ok())
            .collect::<Option<HashSet<u32>>>()
            .ok_or("Failed to parse the numbers")?;

        Ok(Card { winning, numbers })
    }

    fn points(&self) -> u32 {
        let intersection = self.winning.intersection(&self.numbers);
        intersection.count() as u32
    }

    fn worth(&self) -> u32 {
        let points = self.points();
        match points {
            0 => 0,
            _ => 1 << (points - 1),
        }
    }
}

fn main() {
    let file = read_to_string(FILE).expect("Faild to read file");
    let lines = file.lines();
    let cards = lines
        .map(|line| Card::from(line))
        .collect::<Result<Vec<Card>, &str>>()
        .expect("Failed to parse the cards");

    let part1 = cards.iter().map(|card| card.worth()).sum::<u32>();
    println!("Part 1: {}", part1);

    let mut occurences = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in i + 1..=i + card.points() as usize {
            occurences[j] += occurences[i];
        }
    }

    let part2 = occurences.iter().sum::<u32>();
    println!("Part 2: {}", part2);
}
