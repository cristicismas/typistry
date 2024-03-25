use std::io::stdout;
use std::{fs::read_to_string, io};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, size},
    ExecutableCommand,
};

const WORDS_PER_ROW: u8 = 7;
const WORDS_TO_SPAWN: u8 = 7;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let (cols, rows) = size()?;

    let halfway_cols = cols / 2;

    execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    let words = read_lines("./src/words.txt")?;

    println!("{:?}", words);
    Ok(())
}

fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let mut result = Vec::new();

    for line in read_to_string(filename)?.lines() {
        result.push(line.to_string())
    }

    Ok(result)
}
