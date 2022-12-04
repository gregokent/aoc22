mod day1;
mod day2;
mod day3;
mod day4;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, crate::Error>;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    day3::run()?;
    day4::run()?;

    Ok(())
}
