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

            if let Ok(Event::Key(key_event)) = read() {

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
            }

            execute!(
                stdout(),
                MoveTo(0, (self.number_row + 1) as u16),
                Clear(ClearType::CurrentLine),
                Print(format!("{}: {}", message, text)),
                MoveTo((message.len() + 2 + cursor) as u16, (self.number_row+1) as u16)
            ).unwrap();

        }

        Some(text)
    }

}