use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}
