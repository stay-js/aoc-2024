use std::fs::read_to_string;

fn first_part(input: &str) -> u64 {
    let mut disk = Vec::new();
    let mut id = 0;

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Some(id); count]);
            id += 1;
        } else {
            disk.extend(vec![None; count]);
        }
    }

    let mut leftmost_space = disk.iter().position(|item| matches!(item, None)).unwrap();
    let mut rightmost_file = disk
        .iter()
        .rposition(|item| matches!(item, Some(_)))
        .unwrap();

    while leftmost_space < rightmost_file {
        disk.swap(leftmost_space, rightmost_file);

        while disk[leftmost_space].is_some() {
            leftmost_space += 1;
        }

        while disk[rightmost_file].is_none() {
            rightmost_file -= 1;
        }
    }

    return disk
        .iter()
        .enumerate()
        .filter_map(|(idx, &entry)| entry.map(|id| idx * id))
        .sum::<usize>() as u64;
}

struct Block {
    start: usize,
    length: usize,
}

impl Block {
    fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
}

fn second_part(input: &str) -> u64 {
    let mut disk = Vec::new();

    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut id = 0;
    let mut index = 0;

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Some(id); count]);
            files.push(Block::new(index, count));
            id += 1;
        } else {
            disk.extend(vec![None; count]);
            spaces.push(Block::new(index, count));
        }

        index += count;
    }

    for file in files.iter().rev() {
        for space in spaces.iter_mut() {
            if space.length >= file.length && file.start > space.start {
                for i in 0..file.length {
                    disk.swap(space.start + i, file.start + i);
                }

                space.start += file.length;
                space.length -= file.length;
                break;
            }
        }
    }

    return disk
        .iter()
        .enumerate()
        .filter_map(|(idx, &entry)| entry.map(|id| idx * id))
        .sum::<usize>() as u64;
}

fn main() {
    let demo_input = read_to_string("demo-input.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    println!("demo-input:");
    println!("{}", first_part(&demo_input));
    println!("{}", second_part(&demo_input));

    println!("\ninput:");
    println!("{}", first_part(&input));
    println!("{}", second_part(&input));
}
