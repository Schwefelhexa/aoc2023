use std::collections::HashMap;

use anyhow::Result;

pub fn run_day(input: &str) -> Result<()> {
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let part_two = true;

    let mappings = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let rows = input.lines().map(|r| {
        if !part_two {
            return r.to_string();
        }

        let mut row = String::new();
        let mut idx = 0;
        while idx < r.len() {
            let r = &r[idx..r.len()];

            if "0123456789".contains(&r[0..=0]) {
                row.push_str(&r[0..=0]);
            }

            for key in mappings.keys() {
                if r.starts_with(key) {
                    row.push_str(mappings[key]);
                    break;
                }
            }

            idx += 1;
        }
        row
    });
    let digits_only = rows
        .map(|r| r.replace(&alphabet.as_slice()[..], ""))
        .map(|r| {
            // Only take first and last char (which at this point are guaranteed to be digits)
            format!(
                "{}{}",
                r.get(0..=0).unwrap(),
                r.get((r.len() - 1)..r.len()).unwrap()
            )
        });
    let digits: Vec<_> = digits_only
        .map(|n| n.parse::<u32>())
        .filter_map(|n| n.ok())
        .collect();

    let sum: u32 = digits.iter().sum();
    println!("{sum}");

    Ok(())
}

