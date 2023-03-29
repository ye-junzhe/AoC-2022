// Puzzle from: https://adventofcode.com/2022/day/8

fn main() {
    // println!(
    //     "{:?}\n-----------------------",
    //     parse().iter().for_each(|line| {
    //         println!("{:?}", line);
    //     })
    // );
    // println!("Starts iteration at (1,1)");
    println!("{:?}", part_1());
    println!("{:?}", part_2());
}

fn parse() -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string("/Users/mcf4r/Dev/aoc_2022/day_8/input.txt").unwrap();
    // println!(
    //     "\n**NOTE**: when using split_at(), the number you pass in subtract 1 is the index in that container\n"
    // );
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

/*
here we get a vec of 4 vecs of each direction

            up
    left    x,y    right
            down

    [up, down, left, right]
*/
fn directions(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();
    // println!("row: {:?}", row);
    // println!("column: {:?}", column);
    //
    // println!("Starts iterating at({x},{y})");

    let (left, right) = row.split_at(x);
    let (up, down) = column.split_at(y);
    // println!("Just after split:");
    // println!("{:?}", left);
    // println!("{:?}", right);
    // println!("{:?}", up);
    // println!("{:?}", down);

    // .rev() is to reverse the upside of the (x,y) as we see the trees in the perspective of (x,y)
    // same to the left side
    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    // and also why we the index starts at 1 for right and down
    // bcz after split_at() we have two []s
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    // println!("After removing self:");
    // println!("{:?}", left);
    // println!("{:?}", right);
    // println!("{:?}", up);
    // println!("{:?}", down);
    // println!("-----------------");
    [up, down, left, right]
}

fn part_1() -> usize {
    let trees = parse();
    let len = trees.len();

    let mut count = 0;

    for y in 1..len - 1 {
        for x in 1..len - 1 {
            let height = trees[y][x];
            let directions = directions(&trees, x, y);
            let mut visible = false;

            for direction in &directions {
                if direction.iter().all(|h| *h < height) {
                    visible = true;
                    break;
                }
            }

            if visible {
                count += 1;
            }
        }
    }

    count + (len - 1) * 4
}

fn part_2() -> usize {
    let trees = parse();
    let len = trees.len();

    let mut max_product = 0;

    for y in 1..len - 1 {
        for x in 1..len - 1 {
            let height = trees[y][x];
            let directions = directions(&trees, x, y);
            let mut product = 1;

            for direction in &directions {
                let position = direction.iter().position(|h| *h >= height);

                if let Some(p) = position {
                    product *= p + 1;
                } else {
                    product *= direction.len();
                }
            }

            if product > max_product {
                max_product = product;
            }
        }
    }

    max_product
}
