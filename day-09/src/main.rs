use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Block {
    File(u16),
    FreeSpace,
}

fn get_leftmost_free_space(disk: &Vec<Block>) -> Option<usize> {
    return disk
        .iter()
        .position(|item| matches!(item, Block::FreeSpace));
}

fn get_rightmost_file(disk: &Vec<Block>) -> Option<usize> {
    return disk.iter().rposition(|item| matches!(item, Block::File(_)));
}

fn first_part(input: &str) -> u64 {
    let mut index = 0;
    let mut disk = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let count = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            disk.extend(vec![Block::File(index); count]);

            index += 1;
            continue;
        }

        disk.extend(vec![Block::FreeSpace; count]);
    }

    while let (Some(a), Some(b)) = (get_leftmost_free_space(&disk), get_rightmost_file(&disk)) {
        if a > b {
            break;
        }

        (disk[a], disk[b]) = (disk[b], disk[a]);
    }

    return disk.iter().enumerate().fold(0, |acc, (idx, item)| {
        if let Block::File(file) = item {
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
