use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

enum Flow {
    INCREMENT,
    DECREMENT,
}

fn main() -> io::Result<()> {
    let file_path = "./input.txt";

    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut count = 0;

    'outer: for line in reader.lines() {
        let line: String = line?;

        let report_in_string: Vec<&str> = line.split_whitespace().collect();

        let report_in_number: Result<Vec<i32>, _> =
            report_in_string.iter().map(|s| s.parse::<i32>()).collect();

        match report_in_number {
            Ok(report) => {
                let flow: Option<Flow> = if report[1] - report[0] > 0 {
                    Some(Flow::INCREMENT)
                } else if report[1] - report[0] < 0 {
                    Some(Flow::DECREMENT)
                } else {
                    None
                };

                for n in 1..report.len() {
                    match flow {
                        Some(Flow::INCREMENT) => {
                            if report[n] <= report[n - 1]
                                || (report[n] - report[n - 1]).abs() < 1
                                || (report[n] - report[n - 1]).abs() > 3
                            {
                                continue 'outer;
                            }
                        }
                        Some(Flow::DECREMENT) => {
                            if report[n] >= report[n - 1]
                                || (report[n] - report[n - 1]).abs() < 1
                                || (report[n] - report[n - 1]).abs() > 3
                            {
                                continue 'outer;
                            }
                        }
                        None => {
                            continue 'outer;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error parsing string to number: {}", e);
            }
        }

        count += 1;
    }

    println!("Count: {count}");

    Ok(())
}
