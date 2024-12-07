use std::fs::read_to_string;

fn create_variations(numbers: Vec<u64>, current: Vec<u64>, concat: bool) -> Vec<u64> {
    if numbers.len() == 0 {
        return current;
    }

    if current.len() == 0 {
        return create_variations(numbers[1..].to_vec(), vec![numbers[0]], concat);
    }

    let mut new: Vec<u64> = Vec::new();

    for c in current {
        new.push(c + numbers[0]);
        new.push(c * numbers[0]);

        if concat {
            new.push(format!("{}{}", c, numbers[0]).parse().unwrap());
        }
    }

    return create_variations(numbers[1..].to_vec(), new, concat);
}

fn calculate_total(input: &String, concat: bool) -> u64 {
    return input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();

            let left = parts[0].parse().unwrap();
            let right: Vec<u64> = parts[1]
                .split(" ")
                .map(|part| part.parse().unwrap())
                .collect();

            return (left, right);
        })
        .filter(|eq| create_variations(eq.1.clone(), Vec::new(), concat).contains(&eq.0))
        .map(|eq| eq.0)
        .sum();
}

fn first_part(input: &String) -> u64 {
    return calculate_total(input, false);
}

fn second_part(input: &String) -> u64 {
    return calculate_total(input, true);
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
