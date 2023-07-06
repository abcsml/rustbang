use std::{io::{self, Stdout}, time::Duration, collections::VecDeque};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, poll, Event, MouseEventKind, read, MouseButton, DisableMouseCapture}};
use tui::{Terminal, backend::CrosstermBackend, widgets::{Paragraph, Block, Borders, Wrap}, style::{Color, Style}, layout::{Alignment, Layout, Direction, Constraint}, text::{Spans, Span}};

use crate::{base::{Game, Step, Board, Player, self, Role}, ai::{self, AI}};

use super::{util, Display, log};

static mut TEXT: Vec<Spans> = Vec::new();

pub unsafe fn tui_log(spans: Spans<'static>) {
    if TEXT.len() > 20 {
        // TEXT.pop_front();
        TEXT.remove(TEXT.len()-1);
    }
    TEXT.push(spans);
    // TEXT.push_back(spans);
}

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

pub fn tui_draw<M: Display>(terminal: &mut Terminal<CrosstermBackend<Stdout>>, maps: Vec<&M>) {
    let _ = terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
                ].as_ref()
            )
            .split(f.size());

        {
            let p = Paragraph::new(util::generate_map(maps[0]))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))//.bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });

            f.render_widget(p, chunks[0]);
        }

        {
            let p = Paragraph::new(unsafe { TEXT.clone() })
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))//.bg(Color::Black))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: false });

            f.render_widget(p, chunks[1]);
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

impl<B: Board<S>, S: Step + std::fmt::Debug> Game<B, S> {
    pub fn tui_main(&mut self) where B: Display + AI<S> {
        let mut tem = tui_init().unwrap();
        loop {
            // if let base::GameState::Over(_o) = game.state {
                // match o {
                //     base::OutCome::Draw
                // }
            // }
            tui_draw(&mut tem, vec![&self.board]);

            match self.players[self.curr_player.0 as usize] {
                Role::Com => {
                    let a = self.step(ai::get_next_best_step(&self.board, self.curr_player).unwrap());
                    if a {
                        log(self.board.to_string());
                    }
                }
                Role::Hum => {
                    let event = tui_get_event();
                    if let TuiEvent::Exit = event {
                        break;
                    }

                    if let TuiEvent::GetPos((x, y)) = event {
                        let step = S::new(x as u8, y as u8, self.curr_player);
                        if self.state == base::GameState::Running {
                            let _ = self.step(step);
                            log(self.board.to_string());
                        }
                    }
                }
            }
        }

        let _ = tui_exit(&mut tem);
    }
}
