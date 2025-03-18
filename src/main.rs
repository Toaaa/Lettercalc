use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use rayon::prelude::*;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

const VALID_RATING_MIN: f64 = 0.5;
const VALID_RATING_MAX: f64 = 5.0;
const RATING_STEP: f64 = 0.5;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File containing the ratings
    #[arg(short = 'f', long = "file")]
    file: Option<String>,

    /// Ratings provided manually
    #[arg(short = 'r', long = "ratings")]
    ratings: Option<String>,

    /// Allow arbitrary rating steps (not restricted to 0.5 steps)
    #[arg(short = 'x', long = "flexible")]
    flexible: bool,
}

fn read_ratings_from_file(filename: &str, flexible: bool) -> Result<Vec<f64>> {
    let file = File::open(filename)
        .with_context(|| format!("Failed to open file: {}", filename))?;
    let reader = BufReader::new(file);
    let mut ratings = Vec::new();

    for line in reader.lines() {
        let line = line.context("Failed to read line from file")?;
        let rating = line.trim().parse::<f64>().with_context(|| {
            format!("Failed to parse line as a number: {}", line)
        })?;

        if is_valid_rating(rating, flexible) {
            ratings.push(rating);
        } else {
            eprintln!(
                "Warning: Rating {} outside of valid range ({}-{}) or invalid step",
                rating, VALID_RATING_MIN, VALID_RATING_MAX
            );
        }
    }

    Ok(ratings)
}

fn process_ratings_input(ratings_input: &str, flexible: bool) -> Result<Vec<f64>> {
    ratings_input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<f64>()
                .with_context(|| format!("Failed to parse rating: {}", s))
                .and_then(|rating| {
                    if is_valid_rating(rating, flexible) {
                        Ok(rating)
                    } else {
                        Err(anyhow::anyhow!(
                            "Invalid rating: {}. Ratings must be in the range ({}, {}) and steps must match",
                            rating, VALID_RATING_MIN, VALID_RATING_MAX
                        ))
                    }
                })
        })
        .collect()
}

fn is_valid_rating(rating: f64, flexible: bool) -> bool {
    if rating < VALID_RATING_MIN || rating > VALID_RATING_MAX {
        return false;
    }

    if flexible {
        return true;
    }

    (rating * 10.0).round() % (RATING_STEP * 10.0) == 0.0
}

fn calculate_average_rating(ratings: &[f64]) -> Option<f64> {
    if ratings.is_empty() {
        None
    } else {
        let sum: f64 = ratings.par_iter().copied().sum();
        Some(sum / ratings.len() as f64)
    }
}

fn convert_to_star_rating(average: f64) -> String {
    let full_stars = average.floor() as usize;
    let fractional_part = average - full_stars as f64;

    let fractional_star = match fractional_part {
        0.0..=0.39 => "",
        0.39..=0.74 => "½",
        0.75..=1.0 => "★",
        _ => "",
    };

    let mut star_string = String::with_capacity(full_stars + fractional_star.len());
    star_string.push_str("★".repeat(full_stars).as_str());
    star_string.push_str(fractional_star);

    star_string
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut cmd = Args::command();

    let file_path = match args.file {
        Some(ref file) => file.clone(),
        None => {
            let current_dir = get_current_working_dir()?.to_str().unwrap_or("").to_string();
            let default_file_path = format!("{}/ratings.txt", current_dir);
            default_file_path
        }
    };

    let ratings = if let Some(ratings_input) = args.ratings {
        process_ratings_input(&ratings_input, args.flexible)?
    } else {
        if !Path::new(&file_path).exists() {
            eprintln!("Error: File '{}' not found!", file_path);
            let _ = cmd.print_help();
            std::process::exit(1);
        }
        read_ratings_from_file(&file_path, args.flexible)?
    };

    match calculate_average_rating(&ratings) {
        Some(average_rating) => {
            let star_representation = convert_to_star_rating(average_rating);
            println!("⌀: {} ({:.2})", star_representation, average_rating);
        }
        None => {
            println!("No valid ratings found.");
            let _ = cmd.print_help();
        }
    }

    Ok(())
}
