use std::{collections::HashSet, fs::read_to_string};

type Position = (usize, usize);

fn first_part(input: &str) -> u32 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                sum += calculate_score(&map, (j, i), &mut HashSet::new());
            }
        }
    }

    return sum;
}

fn calculate_score(map: &Vec<Vec<u8>>, pos: Position, visited: &mut HashSet<Position>) -> u32 {
    if visited.contains(&pos) {
        return 0;
    }

    visited.insert(pos);

    let (x, y) = pos;

    if map[y][x] == 9 {
        return 1;
    }

    let mut score = 0;

    let height = map.len() as isize;
    let width = map[0].len() as isize;

    for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && nx < width && ny < height {
            let nx = nx as usize;
            let ny = ny as usize;

            if map[ny][nx] == map[y][x] + 1 {
                score += calculate_score(map, (nx, ny), visited);
            }
        }
    }

    return score;
}

fn main() {
    let demo_input = read_to_string("demo-input.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    println!("demo-input:");
    println!("{}", first_part(&demo_input));
    // println!("{}", second_part(&demo_input));

    println!("\ninput:");
    println!("{}", first_part(&input));
    // println!("{}", second_part(&input));
}
