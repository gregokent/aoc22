mod day1;
mod day2;
mod day3;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    day3::run()?;

    Ok(())
}
