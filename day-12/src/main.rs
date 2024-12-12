use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn first_part(input: &str) -> u32 {
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut fields: Vec<(u32, u32)> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            visited.insert((x, y));

            let mut area = 0;
            let mut perimeter = 0;

            while let Some((cx, cy)) = queue.pop_front() {
                area += 1;

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = cx as isize + dx;
                    let ny = cy as isize + dy;

                    if nx < 0 || ny < 0 || nx >= width || ny >= height {
                        perimeter += 1;
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    let neighbor = map[ny][nx];

                    if &neighbor != cell {
                        perimeter += 1;
                        continue;
                    }

                    if !visited.contains(&(nx, ny)) {
                        visited.insert((nx, ny));
                        queue.push_back((nx, ny));
                    }
                }
            }

            fields.push((area, perimeter));
        }
    }

    return fields
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum();
}

fn main() {
    let demo_input = read_to_string("demo-input.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    println!("demo-input:");
    println!("{}", first_part(&demo_input));

    println!("\ninput:");
    println!("{}", first_part(&input));
}
