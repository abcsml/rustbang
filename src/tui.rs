mod display;

use std::{io, thread, time::{Duration, Instant}};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, BorderType, Table, Row, Cell, Wrap},
    layout::{Layout, Constraint, Direction, Alignment, Rect},
    Terminal, text::{Text, Span, Spans}, style::{Modifier, Style, Color}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, read, KeyEvent, MouseEvent, MouseEventKind, MouseButton, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{tui::display::get_pos, board::{TTTBoard, self, Role}, score::TTTSCORE, ai::AI};

use self::display::generate_board;

// pub fn tui_init() -> Result<(), io::Error> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
// }

pub fn tui_main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Block")
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;

    // thread::sleep(Duration::from_millis(5000));
    // let events = Event::new();
    let mut board = TTTBoard::new();
    let mut r = Role::Hum;
    let mut flag = false;
    
    let start_time = Instant::now();
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(10)
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref()
                )
                .split(f.size());
            // let block = Block::default()
            //     .title("Block")
            //     .borders(Borders::ALL);
            // f.render_widget(block, chunks[0]);
            // let block = Block::default()
            //     .title("Block 2")
            //     .borders(Borders::ALL);
            // f.render_widget(block, chunks[1]);
            let size = f.size();
            // let block = Block::default()
            //     .borders(Borders::ALL)
            //     .title("2D Mofang game")
            //     .border_type(BorderType::Rounded);
            // f.render_widget(block, size);
            // let chunks = Layout::default()
            //     .direction(Direction::Vertical)
            //     .constraints(
            //         [
            //             Constraint::Percentage(10),
            //             Constraint::Percentage(80),
            //             Constraint::Percentage(10),
            //         ]
            //         .as_ref(),
            //     )
            //     .split(f.size());
            {
                let text = vec![
                    Spans::from(vec![
                        Span::raw("First"),
                        Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
                        Span::raw("."),
                    ]),
                    Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
                ];
                let x = Span::raw("x");

                let p = Paragraph::new(generate_board(board))
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: false });

                // f.render_widget(p, Rect{x:0,y:0,width:5,height:3});
                f.render_widget(p, size);
            }
        })?;

        // if let Event::Key(key) = read()? {
        //     if key.code == KeyCode::Char('q') {
        //         break;
        //     }
        // }

        if poll(Duration::from_millis(100))? {
            if let Event::Mouse(mouse) = read()? {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    // println!("col: {}, row: {}", mouse.column, mouse.row);
                    // println!("x: {}, y: {}", get_pos(mouse.column, mouse.row).0, get_pos(mouse.column, mouse.row).1);
                    // let p = get_pos(mouse.column, mouse.row);
                    // if p != None {
                    //     board.put(p.unwrap(), Role::Hum);
                    //     // r = r.rev();
                    //     AI::put(&mut board);
                    // }
                    let p = get_pos(mouse.column, mouse.row);
                    if p.is_some() && board.can_put(p.unwrap()) {
                        board.put(p.unwrap(), Role::Hum);
                        // r = r.rev();
                        flag = true;
                    }
                }
                if mouse.kind == MouseEventKind::Down(MouseButton::Right) {
                    break;
                }
            }
        }

        if flag {
            AI::put(&mut board);
        }
        flag = false;
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}