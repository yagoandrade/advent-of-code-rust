pub fn part01(input: &String) {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        left_numbers.push(numbers[0].parse().unwrap());
        right_numbers.push(numbers[1].parse().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let total_distance: i32 = get_total_distance(&left_numbers, &right_numbers);

    println!("DAY 01.1: Total distance: {}", total_distance);
}

pub fn part02(input: &String) {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        left_numbers.push(numbers[0].parse().unwrap());
        right_numbers.push(numbers[1].parse().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();

    let mut total_similarity_score = 0;
    let similarity_score = calculate_similarity_scores(&left_numbers, &right_numbers);

    similarity_score.iter().for_each(|(_, value)| {
        total_similarity_score += value;
    });

    println!(
        "DAY 01.2: Total similarity score: {}",
        total_similarity_score
    );
}

fn get_total_distance(left_numbers: &[i32], right_numbers: &[i32]) -> i32 {
    left_numbers
        .iter()
        .zip(right_numbers)
        .map(|(&x, &y)| (x - y).abs())
        .sum()
}

use std::collections::HashMap;
fn calculate_similarity_scores(left_numbers: &[i32], right_numbers: &[i32]) -> HashMap<i32, i32> {
    let mut similarity_score: HashMap<i32, i32> = HashMap::new();

    for &left_number in left_numbers {
        let times_left_is_equal_to_right = right_numbers
            .iter()
            .filter(|&&right_number| right_number == left_number)
            .count() as i32;

        let score = left_number * times_left_is_equal_to_right;

        *similarity_score.entry(left_number).or_default() += score;
    }

    similarity_score
}
