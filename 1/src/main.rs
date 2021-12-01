use std::fs;

fn task1(depths: &Vec<u32>) -> u32 {
    let mut result = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            result += 1;
        }
    }

    result
}

fn task2(depths: &Vec<u32>) -> u32 {
    let max_iters = 3 * (depths.len() as f32 / 3.0).floor() as usize;
    let mut result = 0;
    let mut prev_sum: u32 = depths[0..3].iter().sum();
    
    for i in 1..max_iters {
        let sum: u32 = depths[i..(i+3)].iter().sum();
        if sum > prev_sum {
            result += 1;
        }

        prev_sum = sum;
    }

    result
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let entries = content.lines();
    let depths: Vec<u32> = entries
        .map(|e| e.parse().unwrap())
        .collect();

    println!("Task 1: {}", task1(&depths));
    println!("Task 2: {}", task2(&depths));
}
