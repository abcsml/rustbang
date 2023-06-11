use tui::{text::{Text, Span, Spans}};

use crate::board::{Pos, TTTBoard};

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

pub fn generate_board(board: TTTBoard) -> Vec<Spans<'static>> {
    let size = TTTBoard::size();
    // let data = Span::raw("x");
    let vl = Span::raw(" │ ");
    let hl = Span::raw("─");
    let cr = Span::raw("┼───");

    let mut text = vec![];
    for i in 0..size {
        let mut line = vec![Span::raw(" ")];
        for j in 0..size {
            line.push(Span::raw(board.pieces[i as usize][j as usize].to_char().to_string()));
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

pub fn get_pos(col: u16, row: u16) -> Option<Pos> {
    let mut p = None;
    if row % 2 == 1 && col % 4 != 0 {
        let x = (row-1)/2;
        let y = col / 4;
        p = Some(Pos(x as i8, y as i8));
    }

    p
}
