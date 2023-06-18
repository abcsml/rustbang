use std::{io::{self, Stdout}, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, poll, Event, MouseEventKind, read, MouseButton, DisableMouseCapture}};
use tui::{Terminal, backend::CrosstermBackend, widgets::{Paragraph, Block, Borders, Wrap}, style::{Color, Style}, layout::Alignment};

use super::{util, Display};

pub enum TuiEvent {
    None,
    GetPos((u16, u16)),
    Exit,
}

pub fn tui_init() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

pub fn tui_draw<B: Display>(terminal: &mut Terminal<CrosstermBackend<Stdout>>, board: B) {
    terminal.draw(|f| {
        let size = f.size();
        {
            let p = Paragraph::new(util::generate_board(board))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });

            f.render_widget(p, size);
        }
    });
}

pub fn tui_get_event() -> TuiEvent {
    if poll(Duration::from_millis(100)).unwrap() {
        if let Event::Mouse(mouse) = read().unwrap() {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                let pos = util::get_pos(mouse.column, mouse.row);
                if pos != None {
                    return TuiEvent::GetPos(pos.unwrap());
                }
            }
            if mouse.kind == MouseEventKind::Down(MouseButton::Right) {
                return TuiEvent::Exit;
            }
        }
    }
    TuiEvent::None
}

pub fn tui_exit(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
