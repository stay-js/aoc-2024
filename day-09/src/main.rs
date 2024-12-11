use std::fs::read_to_string;

fn first_part(input: &str) -> u64 {
    let mut index: u16 = 0;
    let mut disk = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Some(index); count]);
            index += 1;
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

    return disk.iter().enumerate().fold(0, |acc, (idx, item)| {
        if let Some(file) = item {
            return acc + *file as u64 * idx as u64;
        }

        return acc;
    });
}

fn second_part(input: &str) -> u64 {
    let mut index: u16 = 0;
    let mut disk = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Some((index, count)); count]);
            index += 1;
        } else {
            disk.extend(vec![None; count]);
        }
    }

    let mut files: Vec<_> = disk
        .iter()
        .filter_map(|item| item.as_ref().copied())
        .collect();

    files.dedup();

    for file in files.into_iter().rev() {
        let (_, length) = file;

        let file_idx = disk.iter().position(|item| item == &Some(file)).unwrap();

        let mut space_ptr = 0;
        let mut space_length = 0;

        while space_ptr + space_length < disk.len() && space_length < length {
            if disk[space_ptr + space_length].is_none() {
                space_length += 1;
                continue;
            }

            space_ptr += space_length + 1;
            space_length = 0;
        }

        if space_ptr < file_idx && space_length >= length {
            for i in 0..length {
                disk.swap(file_idx + i, space_ptr + i);
            }
        }
    }

    return disk.iter().enumerate().fold(0, |acc, (idx, item)| {
        if let Some((index, _)) = item {
            return acc + *index as u64 * idx as u64;
        }

        return acc;
    });
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
