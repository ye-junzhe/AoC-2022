// Puzzle from: https://adventofcode.com/2022/day/10

use std::fs;

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: u32 = 3;

#[derive(Debug)]
struct CPU {
    // registers: HashMap<String, i32>,
    register_x: i32,
    cycle_finished: i32,
}

impl CPU {
    fn new() -> Self {
        CPU {
            // registers: HashMap::new(),
            register_x: 1,
            cycle_finished: 0,
        }
    }

    fn tick(&mut self) -> i32 {
        self.cycle_finished += 1;
        self.cycle_finished
    }
}

fn main() {
    part_1();
    println!("Part2 is: \n{}", part_2());
}

fn part_1() {
    let input = fs::read_to_string("/Users/mcf4r/Dev/aoc_2022/day_10/input.txt").unwrap();
    let program = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>());
    let test_nums = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    for num in test_nums {
        let mut cpu = CPU::new();
        for line in program.clone() {
            if cpu.cycle_finished < num {
                cpu.tick();
                if line[0].get(0..4) == Some("addx") && cpu.cycle_finished + 2 <= num {
                    cpu.tick(); // one more time for addx operation
                    cpu.register_x += line[1].parse::<i32>().unwrap();
                }
            }
        }
        sum += cpu.register_x * cpu.cycle_finished;
    }

    println!("Part1 is: {}", sum);
}

fn get_pixel(cycle: usize, x: i32) -> char {
    let curr_col = (cycle - 1) % COLS;
    if (curr_col as i32).abs_diff(x) <= SPRITE_WIDTH / 2 {
        '█'
    } else {
        ' '
    }
}

pub fn part_2() -> String {
    let input = std::fs::read_to_string("/Users/mcf4r/Dev/aoc_2022/day_10/input.txt").unwrap();
    let mut x = 1;
    let mut cycle = 1;
    let mut screen = [' '; COLS * ROWS];

    for line in input.lines() {
        screen[cycle - 1] = get_pixel(cycle, x);
        cycle += 1;

        if let Some(("addx", num)) = line.split_once(' ') {
            screen[cycle - 1] = get_pixel(cycle, x);
            cycle += 1;
            let num: i32 = num.parse().unwrap();
            x += num;
        }
    }

    let image = screen
        .chunks(COLS)
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");

    image
}
