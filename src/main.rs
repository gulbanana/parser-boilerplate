use anyhow::{anyhow, Result};
use chumsky::{error::Simple, primitive::*, Parser};
use progress::{interpret, Quest};

mod progress;

fn main() -> Result<()> {
    let src = std::fs::read_to_string(std::env::args().nth(1).ok_or(anyhow!("no args"))?)?;
    println!("{}", src);

    let out = parse(src.as_str());
    let run = interpret(&out.unwrap());
    println!("{}", run);

    Ok(())
}

fn parse(code: &str) -> Result<Vec<Quest>, Vec<Simple<char>>> {
    parse_progress().parse(code)
}

fn parse_whitespace() -> impl Parser<char, Vec<Quest>, Error = Simple<char>> {
    filter(|c: &char| c.is_whitespace()).repeated()
}

fn parse_progress() -> impl Parser<char, Vec<Quest>, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_digit()).map(|_c| vec![])
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::progress::interpret;

    #[test]
    fn test_parser() {}
}
