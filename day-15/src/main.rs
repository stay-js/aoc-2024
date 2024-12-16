use std::fs::read_to_string;

fn get_next_position(direction: &char, x: usize, y: usize) -> Option<(usize, usize)> {
    return match direction {
        '<' if x > 0 => Some((x - 1, y)),
        '>' => Some((x + 1, y)),
        '^' if y > 0 => Some((x, y - 1)),
        'v' => Some((x, y + 1)),
        _ => None,
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

    for direction in directions.iter() {
        if let Some((nx, ny)) = get_next_position(direction, x, y) {
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

            if let Some((mut bx, mut by)) = get_next_position(direction, nx, ny) {
                while matrix[by][bx] == 'O' {
                    if let Some((nbx, nby)) = get_next_position(direction, bx, by) {
                        bx = nbx;
                        by = nby;
                    }
                }

                if matrix[by][bx] == '#' {
                    continue;
                }

                matrix[ny][nx] = '@';
                matrix[y][x] = '.';
                matrix[by][bx] = 'O';
                x = nx;
                y = ny;
            }
        }
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
