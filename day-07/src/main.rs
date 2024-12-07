use std::fs::read_to_string;

type Equation = (u64, Vec<u64>);

fn create_variations(numbers: Vec<u64>, current: Vec<String>) -> Vec<String> {
    if numbers.len() == 0 {
        return current;
    }

    if current.len() == 0 {
        return create_variations(numbers[1..].to_vec(), vec![numbers[0].to_string()]);
    }

    let mut new: Vec<String> = Vec::new();

    for c in current {
        new.push(format!("{} + {}", c, numbers[0]));
        new.push(format!("{} * {}", c, numbers[0]));
    }

    return create_variations(numbers[1..].to_vec(), new);
}

fn is_correct(eq: &Equation) -> bool {
    let variations = create_variations(eq.1.clone(), Vec::new());

    for variation in variations {
        let items: Vec<&str> = variation.split(" ").collect();

        let result: u64 = items.iter().enumerate().fold(0, |acc, (i, item)| {
            if i == 0 {
                return item.parse().unwrap();
            }

            if vec!["+", "*"].contains(item) {
                return acc;
            }

            if items[i - 1] == "+" {
                return acc + item.parse::<u64>().unwrap();
            }

            return acc * item.parse::<u64>().unwrap();
        });

        if result == eq.0 {
            return true;
        }
    }

    return false;
}

fn first_part(input: &String) -> u64 {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();

            let left = parts[0].parse().unwrap();
            let right = parts[1]
                .split(" ")
                .map(|part| part.parse().unwrap())
                .collect();

            return (left, right);
        })
        .collect();

    return equations
        .iter()
        .filter(|eq| is_correct(eq))
        .map(|eq| eq.0)
        .sum();
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
