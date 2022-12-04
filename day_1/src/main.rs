// Puzzle from: https://adventofcode.com/2022/day/1

fn main() {
    println!("Hi! I'm day_1");
    
    let data = include_str!("../input.txt").trim();

    let mut elves: Vec<usize> = data.split("\n\n")
        .map(|elf| elf.split('\n')
            .map(|cal| cal.parse::<usize>().unwrap())
            .sum()
        )
        .collect();


    elves.sort();
    elves.reverse();

    // let max_elf_number = elves.iter().max().unwrap();
    // let max_elf_number_index = elves.iter().position(|r| r == max_elf_number);
    //
    // println!("{:?}",max_elf_number_index);
    // let highest_elf: Vec<usize> = [
    //     2595,
    //     5893,
    //     5590,
    //     6270,
    //     6095,
    //     5279,
    //     3726,
    //     6015,
    //     3985,
    //     5455,
    //     5190,
    //     4897,
    //     6632,
    // ].to_vec();
    // let highest_elf_cal: usize = highest_elf.iter().sum();
    // println!("{:?}", highest_elf);
    // println!("{:?}", highest_elf_cal);
    
    println!("Highest Elf:{}", elves[0]);

    println!("Highest three elves:{}", elves.iter().take(3).sum::<usize>());
}
