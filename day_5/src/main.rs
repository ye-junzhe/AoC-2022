// https://adventofcode.com/2022/day/5

use std::str::Lines;

const INPUT: &str = include_str!("../input.txt");
const STARTING_LINE_OFFSET: usize = 2;

#[derive(Debug)]
struct Dock {
    /// Each stack contains a vector of chars which represent
    /// the creates. The top of the crate stack is represented at
    /// the end of these vectors.
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Instruction {
    amount: u64,
    from_stack: usize,
    to_index: usize,
}

fn main() {
    println!("Hi! I'm day_5");

    let lines = INPUT.lines();

    // Part 1
    let (crate_number_on_this_line, lines_to_build_stacks) =
        find_stack_numbers_which_hold_crate_on_that_index(lines.clone());
    let mut dock = Dock::new(
        lines.clone(),
        crate_number_on_this_line,
        lines_to_build_stacks,
    );
    let instructions = generate_instructions(lines.clone(), lines_to_build_stacks);

    dock.move_crates_with_9000(&instructions);
    let top_crates_string = dock.list_of_crates_on_top();

    println!("Top Crates Strings: {}", top_crates_string);

    // Part 2
    let (crate_number_on_this_line, lines_to_build_stacks) =
        find_stack_numbers_which_hold_crate_on_that_index(lines.clone());
    let mut dock = Dock::new(
        lines.clone(),
        crate_number_on_this_line,
        lines_to_build_stacks,
    );
    let instructions = generate_instructions(lines, lines_to_build_stacks);

    dock.move_crates_with_9001(&instructions);
    let top_crates_string_upgraded = dock.list_of_crates_on_top();

    println!(
        "Top Crates Strings (Upgraded): {}",
        top_crates_string_upgraded
    );
}

/// Returns the amount of stacks which hold crates on that line
/// and the index of the line it was found on.
/// Return: which line, and how many crates are there
fn find_stack_numbers_which_hold_crate_on_that_index(lines: Lines) -> (u64, usize) {
    for (i, line) in lines.into_iter().enumerate() {
        let split = line.split_whitespace();
        let parsed_numbers_count = split.filter(|x| x.parse::<u64>().is_ok()).count() as u64;

        if parsed_numbers_count != 0 {
            return (parsed_numbers_count, i);
        }
    }

    panic!("No parsable line was found!")
}

fn generate_instructions(lines: Lines, lines_to_build_stacks: usize) -> Vec<Instruction> {
    // We add 1 to the index so it gives us the line number of the details line.
    // We then add another one to skip the blank line.
    lines
        .skip(lines_to_build_stacks + STARTING_LINE_OFFSET)
        .map(Instruction::new)
        .collect()
}

impl Dock {
    fn new(lines: Lines, crate_number_on_this_line: u64, lines_to_build_stacks: usize) -> Self {
        let stacks = Self::construct(lines, crate_number_on_this_line, lines_to_build_stacks);
        Self { stacks }
    }

    fn move_crates_with_9000(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            for _ in 0..instruction.amount {
                let c = self.stacks[instruction.from_stack].pop().unwrap();
                self.stacks[instruction.to_index].push(c);
            }
        }
    }

    // Retains the crate order when moving.
    fn move_crates_with_9001(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            let mut crates = Vec::new();

            for _ in 0..instruction.amount {
                let c = self.stacks[instruction.from_stack].pop().unwrap();
                crates.push(c);
            }

            crates.reverse();

            for c in crates {
                self.stacks[instruction.to_index].push(c);
            }
        }
    }

    // The final answer
    fn list_of_crates_on_top(&self) -> String {
        let mut sum_string = String::new();
        for stack in &self.stacks {
            sum_string.push(*stack.last().unwrap());
        }
        sum_string
    }

    fn construct(
        lines: Lines,
        crate_number_on_this_line: u64,
        lines_to_build_stacks: usize,
    ) -> Vec<Vec<char>> {
        // We can use the stack details line index since it is the same as the previous index + 1
        let relevant_lines = lines.take(lines_to_build_stacks);

        // These crates are horizontal but we need to stack them up.
        //
        let crate_rows: Vec<Vec<Option<char>>> =
            relevant_lines.map(Self::crate_line_to_chars).collect();

        let mut stacks = Vec::new();
        for _ in 0..crate_number_on_this_line {
            stacks.push(Vec::new());
        }

        for row in crate_rows.iter().rev() {
            for (i, possible_char) in row.iter().enumerate() {
                if let Some(c) = possible_char {
                    stacks[i].push(*c);
                }
            }
        }

        stacks
    }

    fn crate_line_to_chars(crate_line: &str) -> Vec<Option<char>> {
        let raw_line_chars = crate_line.chars().collect::<Vec<char>>();

        // Each crate in the text is 3 characters long, with a space inbetween.
        // We cannot split on the whitespace as not every stack is full

        let crate_strings = raw_line_chars
            .chunks(4)
            .map(|x| x[0..3].iter().collect::<String>());

        crate_strings
            .map(|x| Self::crate_string_to_char(&x))
            .collect::<Vec<Option<char>>>()
    }

    fn crate_string_to_char(crate_string: &str) -> Option<char> {
        if crate_string.split_whitespace().count() == 0 {
            return None;
        }

        crate_string.chars().nth(1)
    }
}

impl Instruction {
    fn new(line: &str) -> Self {
        let mut split = line.split_whitespace().skip(1).step_by(2);

        let amount = split.next().unwrap().parse().unwrap();

        // We subtract one from here as we're representing the index, not stack number.
        let from_stack = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to_index = split.next().unwrap().parse::<usize>().unwrap() - 1;

        Self {
            amount,
            from_stack,
            to_index,
        }
    }
}
