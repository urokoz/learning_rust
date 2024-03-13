#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn get_num() -> f32 {
    let num: f32 = loop {
        println!("Please input a number:");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Could not read input");

        let num: f32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hey! That's not a number!");
                continue;
            }
        };
        break num;
    };
    num
}

fn read_two_numbers() {
    let num1 = get_num();
    let num2 = get_num();

    let res = (num1 / num2) as i32;
    println!("{res}");
}

/// First attempt of make an an efficient data reader.
/// Crashes if some of the numbers/lines cannot be parsed
/// due to the inner map with unwrap. Here it could be
/// better to use a filter_map, but that also means that
/// if we are expecting a certain formatting then that
/// will not be caught.
fn read_from_file() {
    let f = File::open("data/ex1.dat").expect("This file does not exist.");
    let reader = BufReader::new(f);

    let negatives: usize = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<f32>().unwrap())
                .filter(|x| x < &0.0)
                .count()
        })
        .sum();

    println!("ex1.dat contains {} negative numbers.", negatives)
}

/// Better version of read_from_file() modified by ChatGPT
/// More rubust.
fn read_from_file2() -> Result<(), io::Error> {
    let f = File::open("data/ex1.dat")?;
    let reader = BufReader::new(f);

    let negatives: usize = reader
        .lines()
        .filter_map(Result::ok) // Convert iterator of Result to iterator of Option, filtering out errors
        .flat_map(|line| // Use flat_map to handle each line's numbers
            line.split_whitespace()
                .filter_map(|x| x.parse::<f32>().ok()) // Only keep successfully parsed numbers
                .filter(|&x| x < 0.0)
                .collect::<Vec<f32>>() // Collect to Vec to count negative numbers
                .into_iter())
        .count(); // Count all negative numbers

    println!("ex1.dat contains {} negative numbers.", negatives);
    Ok(())
}

fn find_headers() -> Result<(), std::io::Error> {
    let f = File::open("data/orphans.sp")?;
    let reader = BufReader::new(f);

    let outfile = File::create("accessions.txt")?;
    let mut writer = BufWriter::new(outfile);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| line.starts_with(">"))
        .try_for_each(|header| {
            writeln!(writer, "{}", header[1..].split_whitespace().next().unwrap())
        })
}

fn main() {
    find_headers().unwrap();
}
