// Puzzle from: https://adventofcode.com/2022/day/4

const INPUT: &str = include_str!("../input.txt");

struct Pair {
    first_elf: String,
    second_elf: String,
}

struct FourSections {
    first_elf_start_section: i32,
    first_elf_end_section: i32,

    second_elf_start_section: i32,
    second_elf_end_section: i32,
}

fn main() {
    println!("Hi! I'm day_4");

    let sum = sum(get_sections(get_pairs()));

    println!(
        "The number of fully contain: {},\nThe number of overlap: {}",
        sum[0], sum[1]
    );
}

fn get_pairs() -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    for line in INPUT.lines() {
        let pair = Pair::construct(line);
        pairs.push(pair)
    }
    pairs
}

fn get_sections(pairs: Vec<Pair>) -> Vec<FourSections> {
    let mut four_sections: Vec<FourSections> = Vec::new();

    for pair in pairs {
        let four_section = FourSections::construct(pair);
        four_sections.push(four_section);
    }

    four_sections
}

fn sum(four_sections: Vec<FourSections>) -> Vec<i32> {
    let mut sum_vec: Vec<i32> = Vec::new();
    sum_vec.push(four_sections.iter().map(|x| x.is_contain()).sum());
    sum_vec.push(four_sections.iter().map(|x| x.is_overlap()).sum());
    sum_vec
}

// NOTE: If return bool, use count()
// fn amount_of_fully_overlapping_pairs(all_assignments: &[AssignmentPair]) -> u64 {
//     all_assignments
//         .iter()
//         .filter(|x| x.fully_overlaps())
//         .count() as u64
// }

impl Pair {
    // Divide the pair into two halves, which are assigned by two elves
    fn construct(pair_string: &str) -> Self {
        let pair: Vec<&str> = pair_string.split(",").collect();
        let first_elf = pair[0].to_string();
        let second_elf = pair[1].to_string();

        Self {
            first_elf,
            second_elf,
        }
    }
}

impl FourSections {
    fn construct(divided_pair: Pair) -> Self {
        /*
        NOTE: First Elf
        collect FIRST elf's sections
        */
        let assigned_to_first_elf: Vec<&str> = divided_pair.first_elf.split("-").collect();

        // Used both in elf1 and elf2
        let mut start_section = assigned_to_first_elf[0].to_string();
        let mut end_section = assigned_to_first_elf[1].to_string();

        let first_elf_start_section: i32 = start_section.parse().unwrap();
        let first_elf_end_section: i32 = end_section.parse().unwrap();

        /*
        NOTE: Second Elf
        collect SECOND elf's sections
        */
        let assigned_to_second_elf: Vec<&str> = divided_pair.second_elf.split("-").collect();

        start_section = assigned_to_second_elf[0].to_string();
        end_section = assigned_to_second_elf[1].to_string();

        let second_elf_start_section: i32 = start_section.parse().unwrap();
        let second_elf_end_section: i32 = end_section.parse().unwrap();

        Self {
            first_elf_start_section,
            first_elf_end_section,
            second_elf_start_section,
            second_elf_end_section,
        }
    }

    fn is_contain(&self) -> i32 {
        // NOTE: To contain
        // (3<=1 && 2<=4) or (1<=3 && 4<=2)
        let overlap: i32;

        if (self.first_elf_start_section >= self.second_elf_start_section
            && self.first_elf_end_section <= self.second_elf_end_section)
            || (self.first_elf_start_section <= self.second_elf_start_section
                && self.first_elf_end_section >= self.second_elf_end_section)
        {
            overlap = 1;
        } else {
            overlap = 0;
        }

        overlap
    }

    fn is_overlap(&self) -> i32 {
        // NOTE: To overlap
        // 2 >= 3 && 4 >= 1
        let overlap: i32;

        if (self.first_elf_end_section >= self.second_elf_start_section)
            && (self.second_elf_end_section >= self.first_elf_start_section)
        {
            overlap = 1;
        } else {
            overlap = 0;
        }

        overlap
    }
}
