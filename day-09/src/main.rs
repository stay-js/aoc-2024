use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum FileOrFreeSpace {
    File(u32),
    FreeSpace,
}

fn get_leftmost_free_space(disk: &Vec<FileOrFreeSpace>) -> Option<usize> {
    for (idx, item) in disk.iter().enumerate() {
        if let FileOrFreeSpace::FreeSpace = item {
            return Some(idx);
        }
    }

    return None;
}

fn get_rightmost_file(disk: &Vec<FileOrFreeSpace>) -> Option<usize> {
    for (idx, item) in disk.iter().enumerate().rev() {
        if let FileOrFreeSpace::File(_) = item {
            return Some(idx);
        }
    }

    return None;
}

fn first_part(input: &str) -> u64 {
    let mut index = 0;
    let mut disk = Vec::new();

    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                disk.push(FileOrFreeSpace::File(index));
            }

            index += 1;
            continue;
        }

        for _ in 0..c.to_digit(10).unwrap() {
            disk.push(FileOrFreeSpace::FreeSpace);
        }
    }

    while let Some(a) = get_leftmost_free_space(&disk) {
        if let Some(b) = get_rightmost_file(&disk) {
            if b < a {
                break;
            }

            (disk[a], disk[b]) = (disk[b], disk[a]);
        }
    }

    return disk.iter().enumerate().fold(0, |acc, (idx, item)| {
        if let FileOrFreeSpace::File(file) = item {
            return acc + *file as u64 * idx as u64;
        }

        return acc;
    });
}

fn main() {
    let demo_input = read_to_string("demo-input.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    println!("demo-input:");
    println!("{}", first_part(&demo_input));

    println!("\ninput:");
    println!("{}", first_part(&input));
}
