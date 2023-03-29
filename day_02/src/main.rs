// Puzzel from: https://adventofcode.com/2022/day/2

/*
    Points gained for each round
                        Points gained if select this hand shape
    A   Rock        X   =>       1
    B   Paper       Y   =>       2
    C   Scissors    Z   =>       3
                        What XYZ really means
                        =>      lose
                        =>      draw
                        =>      win

        Out come if win the round
                Lost 0
                Draw 3
                Win  6
*/
use std::{cmp::Ordering, str::FromStr};

// const RAW_INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    println!("Hi! I'm day_2");

    let data = include_str!("../input.txt").trim();

    println!("I thought what the hints are: {}", process_part1(&data));
    println!("What those hints really mean: {}", process_part2(&data));
}

/*
We can use the integers we assigned to make the comparison and decide which side wins
NOTE: But there are two exceptions, which are
                                 Me: Scissors opponent: Rock
                                 Me: Rock opponent: Scissors
*/
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                // NOTE: parse here is contributed to the FromStr trait
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                None => panic!("moves should be comparable"),
            }
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();
            let opponent_move = moves[0].parse::<Move>().unwrap();
            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    };
                    our_move as u32
                }
                "Y" => 3 + opponent_move as u32,
                "Z" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    6 + our_move as u32
                }
                _ => {
                    panic!("Unexpected response");
                }
            }
        })
        .sum();
    result.to_string()
}
