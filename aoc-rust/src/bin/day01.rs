use std::{env, fs::File, io::{BufRead, BufReader}};

#[derive(Clone)]
struct Top3(u64, u64, u64);

fn update_top3(top3: Top3, new: u64) -> Top3 {
    let mut updated = top3.clone();
    if new > updated.2 {
        updated.2 = new;
    }
    if updated.2 > updated.1 {
        std::mem::swap(&mut updated.2, &mut updated.1);
    }
    if updated.1 > updated.0 {
        std::mem::swap(&mut updated.1, &mut updated.0);
    }
    updated
}

fn part01(file: File) {
    let reader = BufReader::new(file);
    let mut total_calories = 0;
    let mut max_calory = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            max_calory = max_calory.max(total_calories);
            total_calories = 0;
        } else {
            total_calories += trimmed.parse::<u64>().unwrap();
        }
    }

    max_calory = max_calory.max(total_calories);
    println!("{}", max_calory);
}

fn part02(file: File) {
    let reader = BufReader::new(file);
    let mut total_calories = 0;
    let mut top3_calories = Top3(0, 0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        if trimmed.len() == 0 {
            top3_calories = update_top3(top3_calories, total_calories);
            total_calories = 0;
        } else {
            total_calories += trimmed.parse::<u64>().unwrap();
        }
    }

    top3_calories = update_top3(top3_calories, total_calories);
    let top3_sum = top3_calories.0 + top3_calories.1 + top3_calories.2;
    println!("{}", top3_sum);
}

fn main() {
    let fpath = env::args()
        .nth(1)
        .expect("Path to input file is not given!");
    part01(File::open(&fpath).unwrap());
    part02(File::open(&fpath).unwrap());
}
