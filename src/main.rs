mod day1;
mod day2;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;

    Ok(())
}
