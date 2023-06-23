use std::{io::{self, Stdout}, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, poll, Event, MouseEventKind, read, MouseButton, DisableMouseCapture}};
use tui::{Terminal, backend::CrosstermBackend, widgets::{Paragraph, Block, Borders, Wrap}, style::{Color, Style}, layout::Alignment};

use crate::{base::{Game, Step, Board, Player, self, Role}, ai::{self, AI}};

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

pub fn tui_draw<B: Display>(terminal: &mut Terminal<CrosstermBackend<Stdout>>, board: &B) {
    let _ = terminal.draw(|f| {
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

impl<B: Board<S>, S: Step> Game<B, S> {
    pub fn tui_main(&mut self) where B: Display + AI<S> {
        let mut tem = tui_init().unwrap();
        loop {
            // if let base::GameState::Over(_o) = game.state {
                // match o {
                //     base::OutCome::Draw
                // }
            // }
            tui_draw(&mut tem, &self.board);

            match self.players[self.curr_player.0 as usize] {
                Role::Com => {
                    let _ = self.step(ai::get_next_best_step(&self.board, self.curr_player).unwrap());
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
                        }
                    }
                }
            }
        }

        let _ = tui_exit(&mut tem);
    }
}
