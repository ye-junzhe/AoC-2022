// Puzzle from: https://adventofcode.com/2022/day/9

use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

fn part_1() {
    let input = std::fs::read_to_string("/Users/mcf4r/Dev/aoc_2022/day_9/input.txt").unwrap();
    let start = Coord { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    let mut seen = HashSet::new();
    seen.insert(tail);
    let cmds = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>());

    for cmd in cmds {
        // println!("==================================================");
        // println!("Cmd passed to head; {:?}", cmd);
        for _ in 0..cmd[1].parse().unwrap() {
            match cmd[0] {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "R" => head.x += 1,
                "L" => head.x -= 1,
                _ => panic!("tried to move in invalid direction"),
            }
            // determine if head and tail are touching
            // println!("Position of head: {:?}", head);
            let diff = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

            // update tail and insert it into the seen set if needed
            if not_touching {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                seen.insert(tail);
            }
        }
    }
    println!("{}", seen.len());
}

fn part_2() {
    // declare 10 Coord structs with coordinate of {0, 0}, put them all in Vec::knots
    let input = std::fs::read_to_string("/Users/mcf4r/Dev/aoc_2022/day_9/input.txt").unwrap();
    let knot: Coord = Coord { x: 0, y: 0 };
    let mut rope = vec![knot; 10];
    let mut seen = HashSet::new();
    seen.insert(knot); // only for the first position to be held
    let cmds = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>());
    for cmd in cmds {
        for _ in 0..cmd[1].parse().unwrap() {
            match cmd[0] {
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                "R" => rope[0].x += 1,
                "L" => rope[0].x -= 1,
                _ => panic!("tried to move in invalid direction"),
            }

            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                // determine if head and tail are touching
                let diff = Coord {
                    x: rope[head_idx].x - rope[tail_idx].x,
                    y: rope[head_idx].y - rope[tail_idx].y,
                };
                let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

                // update tail and insert it into the seen set if needed
                if not_touching {
                    rope[tail_idx].x += diff.x.signum();
                    rope[tail_idx].y += diff.y.signum();
                    if tail_idx == rope.len() - 1 {
                        seen.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }
    println!("{}", seen.len());
}

fn main() {
    part_1();
    part_2();
}
