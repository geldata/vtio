//! Interactive demo demonstrating vtcmd usage with vtinput.
//!
//! Use arrow keys to move the cursor, 'h' to hide, 's' to show, 'q' to quit.

use std::io::{self, Read, Write};
use vtansi::Encode;
use vtcmd::{
    clear::ClearAll,
    cursor::{HideCursor, MoveTo, ShowCursor},
    screen::{EnterAlternateScreen, LeaveAlternateScreen},
    window::SetTitle,
};
use vtinput::{KeyCode, KeyModifiers, TerminalInputEvent, TerminalInputParser};

struct RawMode {
    original_termios: Option<libc::termios>,
}

impl RawMode {
    fn enable() -> io::Result<Self> {
        #[cfg(unix)]
        unsafe {
            let mut termios = std::mem::zeroed();
            if libc::tcgetattr(libc::STDIN_FILENO, &mut termios) != 0 {
                return Err(io::Error::last_os_error());
            }

            let original_termios = termios;

            libc::cfmakeraw(&mut termios);

            if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                original_termios: Some(original_termios),
            })
        }

        #[cfg(not(unix))]
        {
            Ok(Self {
                original_termios: None,
            })
        }
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(termios) = self.original_termios {
            unsafe {
                libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios);
            }
        }
    }
}

fn write_command<T: Encode>(mut command: T) -> io::Result<()> {
    let mut buf = [0u8; 256];
    let len = command
        .encode(&mut buf)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to encode command"))?;
    io::stdout().write_all(&buf[..len])?;
    io::stdout().flush()
}

fn write_text(text: &str) -> io::Result<()> {
    io::stdout().write_all(text.as_bytes())?;
    io::stdout().flush()
}

fn draw_ui(row: u16, col: u16, visible: bool) -> io::Result<()> {
    write_command(ClearAll)?;
    write_command(MoveTo { row: 1, col: 1 })?;
    write_text("vtcmd Interactive Demo\r\n")?;
    write_text("======================\r\n")?;
    write_text("\r\n")?;
    write_text("Commands:\r\n")?;
    write_text("  Arrow keys  - Move cursor\r\n")?;
    write_text("  h           - Hide cursor\r\n")?;
    write_text("  s           - Show cursor\r\n")?;
    write_text("  c           - Clear screen\r\n")?;
    write_text("  q / Ctrl+C  - Quit\r\n")?;
    write_text("\r\n")?;
    write_text(&format!("Cursor position: row={}, col={}\r\n", row, col))?;
    write_text(&format!("Cursor visible: {}\r\n", visible))?;

    write_command(MoveTo { row, col })?;
    if visible {
        write_command(ShowCursor)?;
    } else {
        write_command(HideCursor)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let _raw_mode = RawMode::enable()?;

    write_command(EnterAlternateScreen)?;
    write_command(SetTitle("vtcmd Interactive Demo"))?;

    let mut cursor_row = 10u16;
    let mut cursor_col = 10u16;
    let mut cursor_visible = true;

    draw_ui(cursor_row, cursor_col, cursor_visible)?;

    let mut parser = TerminalInputParser::new();
    let mut stdin = io::stdin();
    let mut buf = [0u8; 1024];

    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        }

        let mut should_quit = false;
        let mut should_redraw = false;

        parser.feed_with(
            &buf[..n],
            &mut |event: TerminalInputEvent<'_>| match event {
                TerminalInputEvent::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && matches!(key.code, KeyCode::Char('c'))
                    {
                        should_quit = true;
                        return;
                    }

                    match key.code {
                        KeyCode::Char('q') => {
                            should_quit = true;
                        }
                        KeyCode::Char('h') => {
                            cursor_visible = false;
                            should_redraw = true;
                        }
                        KeyCode::Char('s') => {
                            cursor_visible = true;
                            should_redraw = true;
                        }
                        KeyCode::Char('c') => {
                            should_redraw = true;
                        }
                        KeyCode::Up => {
                            cursor_row = cursor_row.saturating_sub(1).max(1);
                            should_redraw = true;
                        }
                        KeyCode::Down => {
                            cursor_row = (cursor_row + 1).min(50);
                            should_redraw = true;
                        }
                        KeyCode::Left => {
                            cursor_col = cursor_col.saturating_sub(1).max(1);
                            should_redraw = true;
                        }
                        KeyCode::Right => {
                            cursor_col = (cursor_col + 1).min(200);
                            should_redraw = true;
                        }
                        _ => {}
                    }
                }
                TerminalInputEvent::Resize(cols, rows) => {
                    cursor_row = cursor_row.min(rows);
                    cursor_col = cursor_col.min(cols);
                    should_redraw = true;
                }
                _ => {}
            },
        );

        if should_quit {
            break;
        }

        if should_redraw {
            draw_ui(cursor_row, cursor_col, cursor_visible)?;
        }
    }

    write_command(ShowCursor)?;
    write_command(LeaveAlternateScreen)?;

    Ok(())
}
