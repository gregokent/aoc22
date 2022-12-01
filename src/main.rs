mod day1;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    day1::run()?;

    Ok(())
}
