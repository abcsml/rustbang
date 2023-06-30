use tui::{text::{Text, Span, Spans}, style::{Style, Modifier}};

use super::Display;

// use crate::board::{Pos, TTTBoard};

pub fn generate_empty_board(size: u8) -> Vec<Spans<'static>> {
    let data = Span::raw("x");
    let vl = Span::raw(" │ ");
    let hl = Span::raw("─");
    let cr = Span::raw("┼───");

    let mut text = vec![];
    for i in 0..size {
        let mut line = vec![Span::raw(" ")];
        for j in 0..size {
            line.push(data.clone());
            if j != size-1 {
                line.push(vl.clone());
            }
        }
        text.push(Spans::from(line));

        if i != size-1 {
            line = vec![hl.clone(); 3];
            for _j in 0..size-1 {
                line.push(cr.clone())
            }
            text.push(Spans::from(line));
        }
    }
    text
}

pub fn generate_map<M: Display>(a: &M) -> Vec<Spans<'static>> {
    let size = a.size();
    let arr = a.to_array();
    // let data = Span::raw("x");
    let mut vl = Span::raw(" │ ");
    let mut hl = Span::raw("─");
    let mut cr = Span::raw("┼───");

    let line_style = Style::default()
        .fg(tui::style::Color::LightYellow)
        // .bg(tui::style::Color::Gray)
        .add_modifier(Modifier::DIM);
    vl.style = line_style;
    hl.style = line_style;
    cr.style = line_style;

    let mut text = vec![];
    for i in 0..size.0 {
        let mut line = vec![Span::raw(" ")];
        for j in 0..size.1 {
            let mut ch = Span::raw(arr[i as usize][j as usize].to_string());
            ch.style = Style::default()
                // .fg(tui::style::Color::Red)
                .add_modifier(Modifier::BOLD);
            line.push(ch);
            if j != size.1-1 {
                line.push(vl.clone());
            }
        }
        text.push(Spans::from(line));

        if i != size.0-1 {
            line = vec![hl.clone(); 3];
            for _j in 0..size.1-1 {
                line.push(cr.clone())
            }
            text.push(Spans::from(line));
        }
    }
    text
}

pub fn get_pos(col: u16, row: u16) -> Option<(u16, u16)> {
    let mut p = None;
    if row % 2 == 1 && col % 4 != 0 {
        let x = (row-1)/2;
        let y = col / 4;
        p = Some((x, y));
    }
    p
}
