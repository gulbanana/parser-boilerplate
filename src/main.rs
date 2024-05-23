use anyhow::{anyhow, Result};
mod progress;

fn main() -> Result<()> {
    let src = std::fs::read_to_string(std::env::args().nth(1).ok_or(anyhow!("no args"))?)?;

    println!("{}", src);

    Ok(())
}
