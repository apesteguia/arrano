use ncurses::*;

use crate::{file::ArranoFile, pos::Pos};

const ARRANO_EMPTY_TEXT: &str = "Arrano v0.0.1";
const ARRANO_EMPTY_TEXT2: &str = "Arrano is free and open source";

#[derive(Debug)]
pub struct State {
    pub current: ArranoFile,
    pub win: WINDOW,
    pub screen_pos: Pos<i32>,
    pub buffer_pos: Pos<usize>,
}

impl State {
    pub fn new(path: Option<&String>, dimensions: Pos<i32>) -> std::io::Result<Self> {
        let buffer_pos: Pos<usize> = Pos::new(0, 0);
        let screen_pos = Pos::new(0, 0);

        let win = newwin(dimensions.y, dimensions.x, 0, 0);
        refresh();
        let archivo = match path {
            Some(path) => ArranoFile::new(&path)?,
            None => ArranoFile::new_empty(),
        };

        Ok(State {
            current: archivo,
            win,

            buffer_pos,
            screen_pos,
        })
    }

    pub fn display(&mut self, dimensions: Pos<i32>, display: Pos<i32>) {
        let new_pos = Pos::new(getmaxx(stdscr()), getmaxy(stdscr()));
        if !dimensions.compare(&new_pos) {
            todo!()
        }
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        if self.current.buffer.is_empty() {
            self.display_empty(dimensions, display);
        } else {
            self.display_file(dimensions, display);
        }

        wrefresh(self.win);
    }

    fn display_empty(&self, dimensions: Pos<i32>, display: Pos<i32>) {
        let center_y = dimensions.y / 2;

        let center_x_text1 = (dimensions.x / 2) - (ARRANO_EMPTY_TEXT.len() as i32 / 2);
        let center_x_text2 = (dimensions.x / 2) - (ARRANO_EMPTY_TEXT2.len() as i32 / 2);

        mvwprintw(self.win, center_y - 1, center_x_text1, ARRANO_EMPTY_TEXT);

        mvwprintw(self.win, center_y, center_x_text2, ARRANO_EMPTY_TEXT2);
        for i in display.x..display.y {
            mvwprintw(self.win, i, 0, "~");
        }
    }

    fn display_file(&self, _dimensions: Pos<i32>, display: Pos<i32>) {
        let mut lines = 0;
        for (i, elem) in self.current.buffer.to_owned().into_iter().enumerate() {
            mvwprintw(self.win, i as i32 + 1, 5, &elem);
            let f = format!("{}", i as i32 + 1);
            mvwprintw(self.win, i as i32 + 1, 0, &f);
            lines = i as i32 + 1;
        }
        lines += 1;
        for i in lines..display.y {
            mvwprintw(self.win, i, 0, "~");
        }
        wrefresh(self.win);
    }
}

#[cfg(test)]
pub mod tests {

    #[test]
    fn test_new_state() {}
    #[test]
    fn measuere_save_time() {}
}
