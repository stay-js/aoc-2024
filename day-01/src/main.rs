use std::{fs::read_to_string, iter::zip};

fn get_lists(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(' ').collect();

        left.push(parts.first().unwrap().parse().unwrap());
        right.push(parts.last().unwrap().parse().unwrap());
    });

    left.sort();
    right.sort();

    return (left, right);
}

fn first_part(input: &String) -> u32 {
    let (left, right) = get_lists(input);

    return zip(left, right).fold(0, |acc, (l, r)| acc + u32::abs_diff(l, r));
}

fn second_part(input: &String) -> u32 {
    let (left, right) = get_lists(input);

    return left.iter().fold(0, |acc, l| {
        acc + l * right.iter().filter(|r| &l == r).count() as u32
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
