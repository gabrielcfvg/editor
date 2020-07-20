use crate::Editor;

use std::io::{stdout, Write};
use crossterm::{execute, 
    cursor::{MoveTo}, 
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent},
    style::Print
    
};

impl Editor {

    pub fn prompt(&mut self, message: String) -> Option<String> {

        let mut text = String::new();
        let mut cursor = 0;

        execute!(
            stdout(),
            MoveTo(0, (self.number_row + 1) as u16),
            Clear(ClearType::CurrentLine),
            Print(format!("{}:  (ESC para sair)!", message)),
            MoveTo((message.len() + 2 + cursor) as u16, (self.number_row+1) as u16)
        ).unwrap();

        loop {

            match read().unwrap() {

                Event::Key(key_event) => {
                
                    match key_event {

                        KeyEvent{code: KeyCode::Esc, modifiers: _} => {
                            return None;
                        }

                        KeyEvent{code: KeyCode::Enter, modifiers: _} => {
                            break;
                        }

                        KeyEvent{code: KeyCode::Left, modifiers: _} => {
                            if cursor != 0 {
                                cursor -= 1;
                            }
                        }
                        KeyEvent{code: KeyCode::Right, modifiers: _} => {
                            if cursor < text.len() {
                                cursor += 1;
                            }
                        }

                        KeyEvent{code: KeyCode::Char(ch), modifiers: _} => {
                            if ch.is_ascii() {
                                text.insert(cursor, ch);
                            }
                            cursor += 1;
                        }

                        KeyEvent{code: KeyCode::Backspace, modifiers: _} => {
                            if cursor > 0 {
                                text.remove(cursor-1);
                                cursor -= 1;
                            }
                        }

                        _ => ()
                    }
                },

                Event::Resize(x, y) => {
                    self.number_row = (y-2) as usize;
                    self.number_col = x as usize;

                    self.render_screen(true);
                    self.update_bar();

                },
                _ => ()
            }

            let mut saida = format!("{}: {}", message, text);
            if saida.len() > self.number_col {
                saida = saida[..self.number_col].to_string();
            }


            execute!(
                stdout(),
                MoveTo(0, (self.number_row + 1) as u16),
                Clear(ClearType::CurrentLine),
                Print(saida),
                MoveTo((message.len() + 2 + cursor) as u16, (self.number_row+1) as u16)
            ).unwrap();

        }

        Some(text)
    }

}