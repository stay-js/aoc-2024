use std::fs::read_to_string;

fn get_next_position(direction: &char, x: usize, y: usize) -> (isize, isize) {
    let x = x as isize;
    let y = y as isize;

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
                x = j;
                y = i;
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

        let nx = nx as usize;
        let ny = ny as usize;

        if matrix[ny][nx] == '#' {
            continue;
        }

        if matrix[ny][nx] == '.' {
            matrix[ny][nx] = '@';
            matrix[y][x] = '.';
            x = nx;
            y = ny;
            continue;
        }

        let (mut bx, mut by) = get_next_position(direction, nx, ny);

        while bx >= 0
            && by >= 0
            && by < height
            && bx < width
            && matrix[by as usize][bx as usize] == 'O'
        {
            (bx, by) = get_next_position(direction, bx as usize, by as usize);
        }

        let bx = bx as usize;
        let by = by as usize;

        if matrix[by][bx] == '#' {
            continue;
        }

        matrix[ny][nx] = '@';
        matrix[y][x] = '.';
        matrix[by][bx] = 'O';
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
