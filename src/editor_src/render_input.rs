use crate::Editor;

use std::cmp::{min, max};
use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, Hide, Show}, 
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{SetForegroundColor, Color, Print, ResetColor}
    
};

impl Editor {

    fn recv_key(&mut self) -> Result<Event, Box<dyn std::error::Error>> {

        loop {
            let event = read()?;

            match event {
                Event::Key(_) => return Ok(event),
                Event::Resize(x, y) => {  
                    if x > 5 && y > 5 {
                        self.number_row = (y-2) as usize;
                        self.number_col = x as usize;
                        self.main_loop(true);
                    }
                }
                _ => ()
            };
        }

    }

    pub fn process_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        fn process_key_event(editor: &mut Editor, key_event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {

            match key_event {            

                KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL} |
                KeyEvent{code: KeyCode::Char('x'), modifiers: KeyModifiers::CONTROL} => {
                
                    if editor.modified && editor.quit_number > 0 {
                        editor.set_message(format!("Aperte CTRL-Q mais {} vezes para sair!!!", editor.quit_number), 3);
                        editor.quit_number -= 1;
                        return Ok(());
                    }
                    else {
                        return Err(Box::from("1"));
                    }

                }

                KeyEvent{code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL} => {
                    editor.save()?;
                }

                KeyEvent{code: KeyCode::Enter, modifiers: _} => {
                    editor.new_row();
                }

                KeyEvent{code: KeyCode::Backspace, modifiers: _} => {
                    editor.delete_char();
                }

                KeyEvent{code: KeyCode::Tab, modifiers: _} => {
                    for _ in 0..4 {
                        editor.insert_char(' ');
                    }
                }

                KeyEvent{code: KeyCode::Up, modifiers: _} => {
                    if editor.cursor_y != 0 {
                        editor.cursor_y -= 1;
                    }
                }
                KeyEvent{code: KeyCode::Down, modifiers: _} => {
                    if editor.cursor_y < editor.row_vec.len() {
                        editor.cursor_y += 1;
                    }
                }
                KeyEvent{code: KeyCode::Left, modifiers: _} => {
                    if editor.cursor_x != 0 {
                        editor.cursor_x -= 1;
                    }
                    else if editor.cursor_y > 0 {
                        editor.cursor_y -= 1;
                        editor.cursor_x = editor.row_vec[editor.cursor_y].len();
                    }
                }
                KeyEvent{code: KeyCode::Right, modifiers: _} => {
                    if editor.cursor_y < editor.row_vec.len() {
                        if editor.cursor_x < editor.row_vec[editor.cursor_y].len() {
                            editor.cursor_x += 1;
                        }
                        else {
                            editor.cursor_x = 0;
                            editor.cursor_y += 1;
                        }
                    }
                }
                
                
                KeyEvent{code: KeyCode::Char(ch), modifiers: _} => {
                    if ch.is_ascii() {
                        editor.insert_char(ch);
                    }
                }

                KeyEvent{code: KeyCode::Home, modifiers: _} => {
                    editor.cursor_x = 0
                }

                KeyEvent{code: KeyCode::End, modifiers: _} => {
                    editor.cursor_x = editor.row_vec[editor.cursor_y].len();
                }

                KeyEvent{code: KeyCode::PageDown, modifiers: _} => {
                    editor.cursor_y = min(editor.cursor_y + editor.number_row - 4, editor.row_vec.len());
                }

                KeyEvent{code: KeyCode::PageUp, modifiers: _} => {
                    editor.cursor_y = max(editor.cursor_y - std::cmp::min(editor.number_row-4, editor.cursor_y), 0);
                }

                _ => ()
            }

            editor.quit_number = 4;
            if editor.cursor_y < editor.row_vec.len() {
                if editor.row_vec[editor.cursor_y].len() < editor.cursor_x {
                    editor.cursor_x = editor.row_vec[editor.cursor_y].len();
                }   
            }
            else {
                editor.cursor_x = 0;
            }

            Ok(())
        }
        
        let input = self.recv_key()?;

        match input {
            Event::Key(key_event) => {
                return process_key_event(self, key_event);
            },
            _ => ()
        }

        Ok(())
    }

    pub fn calibrate_positions(&mut self) {
        if self.cursor_y < self.row_off {
            self.row_off = self.cursor_y;
        }
        else if self.cursor_y >= self.row_off + self.number_row {
            self.row_off = self.cursor_y - self.number_row + 1;
        }

        self.render_x = 0;
        if self.cursor_y < self.row_vec.len() {
            self.render_x = self.row_vec[self.cursor_y].cx_to_rx(self.cursor_x);
        }
        if self.render_x < self.col_off {
            self.col_off = self.render_x;
        }
        else if self.render_x >= self.col_off + self.number_col {
            self.col_off = self.render_x - self.number_col + 1;
        }
    }

    pub fn get_color(num: u8) -> Color {

        match num {
            1 => Color::Blue,
            _ => Color::White,
        }
    }

    pub fn render_screen(&mut self, force_all: bool) {

        fn print_line(editor: &mut Editor, cy: usize, init: usize, end: usize) {

            let row = &editor.row_vec[cy];
                
            if row.hlen() != 0 {

                let mut color: u8 = 0;

                for (i, ch) in row.render[init..end].chars().enumerate() {

                    if row.highlight[i] == color {

                        editor.my_stdout.queue(Print(ch)).unwrap();

                    }
                    else {

                        editor.my_stdout.queue(ResetColor).unwrap()
                                        .queue(SetForegroundColor(Editor::get_color(row.highlight[i]))).unwrap()
                                        .queue(Print(ch)).unwrap();

                        color = i as u8;
                    }


                }
            }
            else {

                editor.my_stdout.queue(ResetColor).unwrap()
                                .queue(Print(&row.render)).unwrap();

            }

        }

        let r = self.row_off;
        let c = self.col_off;

        self.calibrate_positions();

        if force_all == false && r == self.row_off && c == self.col_off {

            let idx = self.cursor_y - self.row_off;

            self.my_stdout.queue(MoveTo(0, (idx) as u16)).unwrap()
                     .queue(Hide).unwrap();

            if self.cursor_y >= self.row_vec.len() {
                print!("~");

            }
            else if self.row_vec[self.cursor_y].rlen() >= self.col_off {
                let init = self.col_off;
                let end = min(self.number_col + self.col_off, self.row_vec[self.cursor_y].rlen());

                print_line(self, self.cursor_y, init, end);
                //self.my_stdout.queue(Print(&self.row_vec[self.cursor_y].render[init..end])).unwrap();
            }

            self.my_stdout.queue(Clear(ClearType::UntilNewLine)).unwrap();

        }
        else {

            self.my_stdout.queue(MoveTo(0, 0)).unwrap()
                     .queue(Hide).unwrap();

            for idx in self.row_off..(self.row_off + self.number_row) {

                if idx >= self.row_vec.len() {
                    print!("~");

                }
                else if self.row_vec[idx].rlen() >= self.col_off {
                    let init = self.col_off;
                    let end = min(self.number_col + self.col_off, self.row_vec[idx].rlen());

                    print_line(self, idx, init, end);
                }

                self.my_stdout.queue(Clear(ClearType::UntilNewLine)).unwrap()
                     .queue(Print("\r\n")).unwrap();
                
            }
        }

        self.my_stdout.queue(Show).unwrap()
                 .queue(MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16)).unwrap();

    }



}