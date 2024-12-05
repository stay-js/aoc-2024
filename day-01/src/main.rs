use std::{fs::read_to_string, iter::zip};

fn first_part(input: &String) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(" ").collect();

        left.push(parts.first().unwrap().parse().unwrap());
        right.push(parts.last().unwrap().parse().unwrap());
    });

    left.sort();
    right.sort();

    return zip(left, right).map(|(l, r)| u32::abs_diff(l, r)).sum();
}

fn main() {
    let demo_input = read_to_string("demo-input.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    println!("demo-input:");
    println!("{}", first_part(&demo_input));

    println!("\ninput:");
    println!("{}", first_part(&input));
}
