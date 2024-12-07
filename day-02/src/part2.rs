use std::fs::File;
use std::io::{self, BufRead};
type Report = Vec<i32>;

#[derive(Debug, PartialEq, Eq)]
enum Flow {
    INCREMENT,
    DECREMENT,
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

fn check_safety(report: &Report) -> Safety {
    let mut flow: Option<Flow> = None;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        match diff.signum() {
            -1 => match flow {
                Some(Flow::INCREMENT) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    }
                }
                Some(Flow::DECREMENT) => {
                    return Safety::Unsafe;
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    } else {
                        flow = Some(Flow::INCREMENT)
                    }
                }
            },
            1 => match flow {
                Some(Flow::INCREMENT) => {
                    return Safety::Unsafe;
                }
                Some(Flow::DECREMENT) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Safety::Unsafe;
                    } else {
                        flow = Some(Flow::DECREMENT)
                    }
                }
            },
            0 => {
                return Safety::Unsafe;
            }
            _ => {
                panic!("Oh shiet")
            }
        }
    }
    Safety::Safe
}

fn main() -> io::Result<()> {
    let file_path = "./input.txt";

    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    let mut reports: Vec<Report> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;

        let report_in_string: Vec<&str> = line.split_whitespace().collect();

        let report_in_number: Result<Vec<i32>, _> =
            report_in_string.iter().map(|s| s.parse::<i32>()).collect();

        reports.push(report_in_number.unwrap());
    }

    let result = reports
        .iter()
        .map(|report: &Vec<i32>| {
            if check_safety(report) == Safety::Unsafe {
                for i in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(i);

                    if check_safety(&new_report) == Safety::Safe {
                        return Safety::Safe;
                    } else {
                        continue;
                    }
                }

                return Safety::Unsafe;
            }

            Safety::Safe
        })
        .filter(|safety| safety == &Safety::Safe)
        .count();

    println!("Result: {:?}", result);

    Ok(())
}
