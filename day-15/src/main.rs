use std::fs::read_to_string;

type Position = (isize, isize);

fn get_next_position(direction: &char, x: isize, y: isize) -> Position {
    return match direction {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => panic!("Invalid direction"),
    };
}

fn first_part(input: &String) -> usize {
    let parts: Vec<_> = input.split("\n\n").collect();

    let mut matrix: Vec<Vec<_>> = parts[0]
        .split("\n")
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();

    let directions: Vec<_> = parts[1].replace("\n", "").chars().collect();

    let mut x = 0;
    let mut y = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &'@' {
                x = j as isize;
                y = i as isize;
            }
        }
    }

    let height = matrix.len() as isize;
    let width = matrix[0].len() as isize;

    for direction in directions.iter() {
        let (nx, ny) = get_next_position(direction, x, y);

        if ny < 0 || ny >= height || nx < 0 || nx >= width {
            continue;
        }

        if matrix[ny as usize][nx as usize] == '#' {
            continue;
        }

        if matrix[ny as usize][nx as usize] == '.' {
            matrix[ny as usize][nx as usize] = '@';
            matrix[y as usize][x as usize] = '.';
            x = nx;
            y = ny;
            continue;
        }

        let (mut bx, mut by) = get_next_position(direction, nx, ny);

        while bx >= 0
            && by >= 0
            && by < matrix.len() as isize
            && bx < matrix[0].len() as isize
            && matrix[by as usize][bx as usize] == 'O'
        {
            (bx, by) = get_next_position(direction, bx, by);
        }

        if matrix[by as usize][bx as usize] == '#' {
            continue;
        }

        matrix[ny as usize][nx as usize] = '@';
        matrix[y as usize][x as usize] = '.';
        matrix[by as usize][bx as usize] = 'O';
        x = nx;
        y = ny;
    }

    let mut sum = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &'O' {
                sum += 100 * i + j;
            }
        }
    }

    return sum;
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
