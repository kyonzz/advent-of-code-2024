// https://adventofcode.com/2024/day/1

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "./input.txt";

    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a buffered reader
    let reader = io::BufReader::new(file);

    // Initialize two vectors to store the numbers
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    // Iterate through each line in the file
    for line in reader.lines() {
        // Unwrap the result of reading the line
        let temp = line?;

        // Split the line by space and collect the parts
        let parts: Vec<&str> = temp.split_whitespace().collect();

        // Parse the two parts as integers and push them to the respective arrays
        let num1: i32 = parts[0].parse().unwrap();
        let num2: i32 = parts[1].parse().unwrap();

        // Push the numbers to the arrays
        arr1.push(num1);
        arr2.push(num2);
    }

    arr1.sort();
    arr2.sort();

    let mut result = 0;

    for i in 0..arr1.len() {
        result += (arr1[i] - arr2[i]).abs()
    }

    println!("Result part 1 {:?}", result); // 1938424

    let mut similarity_score = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut count = 0;
    let mut table: HashMap<i32, i32> = HashMap::new();

    while p1 < arr1.len() {
        if arr1[p1] < arr2[p2] {
            p1 += 1;
            count = 0;
            continue;
        }

        if arr1[p1] == arr2[p2] {
            count += 1;
            table.insert(arr1[p1], count);

            p2 += 1;

            continue;
        }

        if arr1[p1] > arr2[p2] {
            p2 += 1;
            count = 0;
            continue;
        }
    }

    for number in arr1 {
        similarity_score += number * table.get(&number).unwrap_or(&0);
    }

    println!("Result part 2 {:?}", similarity_score); // 22014209

    Ok(())
}
