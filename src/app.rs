use crate::{file::ArranoFile, pos::Pos, state::State};
use ncurses::*;
use std::process::exit;

#[derive(Debug)]
pub struct App {
    pub buffers: Vec<State>,
    pub current: usize,
    pub dimensions: Pos<i32>,
    pub display: Pos<i32>,
    pub command: String,
    pub terminado: bool,
}

impl App {
    pub fn new(path: Option<&String>) -> std::io::Result<Self> {
        initscr();
        noecho();
        keypad(stdscr(), true);
        raw();
        start_color();
        cbreak();

        init_color(COLOR_BLACK as i16, 0, 0, 0);
        init_color(COLOR_BLUE as i16, 40, 40, 1000);
        init_color(COLOR_GREEN as i16, 200, 200, 200);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_GREEN);
        init_pair(3, COLOR_WHITE, COLOR_WHITE);

        let dimensions = Pos::new(getmaxx(stdscr()), getmaxy(stdscr()));
        let display = Pos::new(2, dimensions.y - 2);

        let mut buffers = Vec::new();
        buffers.push(State::new(path, dimensions)?);

        Ok(Self {
            current: 0,
            dimensions,
            display,
            buffers,
            command: String::new(),
            terminado: false,
        })
    }

    pub fn display(&mut self) {
        {
            let x = getmaxx(stdscr());
            let y = getmaxy(stdscr());
            if self.dimensions.x != x || self.dimensions.y != y {
                self.dimensions.x = x;
                self.dimensions.y = y;
            }
        }

        self.display_files();
        self.buffers[self.current].display(self.dimensions, self.display);
        let mut i = 1;
        self.buffers.iter().for_each(|f| {
            if f.current.path.is_some() {
                let fmt = f
                    .current
                    .path
                    .as_ref()
                    .to_owned()
                    .unwrap()
                    .to_str()
                    .unwrap();
                mvwprintw(self.buffers[self.current].win, 0, i, &fmt);
                i += fmt.len() as i32 + 1;
            } else {
                let fmt = format!("Empty file");
                mvwprintw(self.buffers[self.current].win, 0, i, &fmt);
                i += fmt.len() as i32 + 1;
            }
        });
    }

    fn display_files(&self) {
        wattron(self.buffers[self.current].win, COLOR_PAIR(2) | A_BOLD());
        mvwhline(self.buffers[self.current].win, 0, 0, 32, self.dimensions.x);
        wattroff(self.buffers[self.current].win, COLOR_PAIR(2) | A_BOLD());
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        keypad(stdscr(), true);
        self.display();

        let mut ch = wgetch(self.buffers[self.current].win);
        while !self.terminado {
            match ch {
                58 => {
                    self.handle_command();
                    self.handle_action().expect("ERROR HANDLE ACTION");
                }
                _ => (),
            }
            self.display();
            ch = wgetch(self.buffers[self.current].win);
        }

        delwin(self.buffers[self.current].win);
        endwin();
        Ok(())
    }

    fn handle_command(&mut self) {
        let x = getmaxx(self.buffers[self.current].win);
        wattron(self.buffers[self.current].win, COLOR_PAIR(2));
        mvwhline(
            self.buffers[self.current].win,
            self.dimensions.y - 2,
            1,
            32,
            x - 2,
        );
        mvwprintw(
            self.buffers[self.current].win,
            self.dimensions.y - 2,
            1,
            ":",
        );
        wattroff(self.buffers[self.current].win, COLOR_PAIR(2));
        self.command.push(':');
        wrefresh(self.buffers[self.current].win);
        let mut terminado = false;

        let mut ch = wgetch(self.buffers[self.current].win);
        while !terminado {
            if ch == KEY_BACKSPACE || ch == 13 {
                if self.command.is_empty() {
                    return;
                }
                self.command.pop();
                mvwhline(
                    self.buffers[self.current].win,
                    self.dimensions.y - 2,
                    1,
                    32,
                    x - 2,
                );
                mvwprintw(
                    self.buffers[self.current].win,
                    self.dimensions.y - 2,
                    1,
                    &self.command,
                );
            } else if ch == '\n' as i32 {
                mvwprintw(
                    self.buffers[self.current].win,
                    self.dimensions.y - 2,
                    1,
                    &self.command,
                );
                terminado = true;
                self.handle_action().expect("ERROR COMMANDO");
            } else {
                self.command.push(char::from_u32(ch as u32).unwrap());
                mvwprintw(
                    self.buffers[self.current].win,
                    self.dimensions.y - 2,
                    1,
                    &self.command,
                );
            }
            wrefresh(self.buffers[self.current].win);
            ch = wgetch(self.buffers[self.current].win);
        }
        self.command.clear();
        wclear(self.buffers[self.current].win);
    }

    fn handle_action(&mut self) -> std::io::Result<()> {
        match self.command.as_str() {
            ":q" => {
                self.buffers.remove(self.current);
                if self.buffers.is_empty() {
                    self.terminado = true;
                    endwin();
                    exit(0);
                } else {
                    self.current -= 1;
                    self.display();
                }
            }
            ":qa" => {
                self.buffers.clear();
                self.terminado = true;
                endwin();
                exit(0);
            }
            ":new" => {
                self.buffers.push(State::new(None, self.dimensions)?);
                self.current += 1;
                self.display();
            }
            _ => {
                let f = format!("Command not found: {}", self.command);
                mvwprintw(self.buffers[self.current].win, self.dimensions.y - 1, 1, &f);
                self.display();
            }
        }

        self.command.clear();

        Ok(())
    }
}
