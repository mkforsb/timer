use crate::Result;
use crate::time::Time;

use std::io;

use crossterm::cursor;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;
use time::Duration;

pub fn draw<W>(w: &mut W, counter: Duration, subtitle: Option<&String>) -> Result<()>
where
    W: io::Write,
{
    let counter_time = Time::from(&counter);
    let size = terminal::size()?;

    let counter_time_formatted = counter_time.format();

    let s = counter_time.render(size);
    execute!(
        w,
        match subtitle {
            Some(text) => terminal::SetTitle(format!("{}: {}", text, counter_time_formatted)),
            None => terminal::SetTitle(counter_time_formatted),
        },
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;

    if let Some(text) = subtitle {
        let left_padded_text = format!("{}{}", " ".repeat((size.0 as usize / 2) - text.len() / 2), text);
        println!("{s}\n{left_padded_text}");
    } else {
        println!("{s}");
    }

    Ok(())
}

pub fn set_up_terminal<W>(w: &mut W) -> std::io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)
}

pub fn restore_terminal<W>(w: &mut W) -> std::io::Result<()>
where
    W: io::Write,
{
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )
}
