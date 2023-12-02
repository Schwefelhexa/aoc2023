use anyhow::Result;

mod day01;
mod day02;
use day02::run_day;

fn main() -> Result<()> {
    let input_path = std::env::args().nth(1).ok_or_else(|| anyhow::anyhow!("Missing input path"))?;
    let input = std::fs::read_to_string(input_path)?;

    run_day(&input)?;

    Ok(())
}
