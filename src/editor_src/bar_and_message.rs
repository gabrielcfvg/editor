use crate::Editor;

use std::time::SystemTime;
use std::io::{stdout, Write};
use std::cmp::min;

use crossterm::{execute, 
    cursor::{MoveTo, Hide, Show}, 
    terminal::{Clear, ClearType},
    style::{SetForegroundColor, SetBackgroundColor, Color, Print, ResetColor}
    
};


impl Editor {

    pub fn update_bar(&mut self) {

        let mut saida = String::new();
        let mut buffer: Vec<String> = vec![];

        buffer.push(format!("[{}]", if self.file != "" {self.file.clone()} else {"sem nome".to_string()}));
        buffer.push(format!("{} linhas", self.row_vec.len()));
        buffer.push(format!("y: {} | x: {}", self.cursor_y+1, self.render_x + 1));
        buffer.push(format!("{} FPS", self.fps));

        if self.modified {
            buffer.push(format!("(modificado)"));
        }

        for item in buffer.iter() {
            if saida.len() > 0 {
                saida.push_str(" | ");
            }
            saida.push_str(item.as_str());
        }
        
        if saida.len() < self.number_col {
            saida.push_str(" ".repeat(self.number_col - saida.len()).as_str());
        }
        saida = saida[..self.number_col].to_string();

        execute!(
            stdout(),
            Hide,
            MoveTo(0, self.number_row as u16),
            Clear(ClearType::CurrentLine),

            SetForegroundColor(Color::Black),
            SetBackgroundColor(Color::White),
            Print(saida),

            ResetColor,
            MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16),
            Show,
        ).unwrap();
    }

    pub fn update_message(&mut self) {  

        if let Some(tm) = self.message_time {
            match tm.0.elapsed() {
                Ok(dur) => {
                    if tm.1 != -1 &&dur.as_secs() > tm.1 as u64{
                        self.message = String::new();
                        self.message_time = None
                    }
                }
                _ => ()
            }
        }

        self.message = self.message[..min(self.message.len(), self.number_col)].to_string();
        execute!(stdout(),
                 Hide, 
                 MoveTo(0, (self.number_row + 1) as u16), 
                 Clear(ClearType::CurrentLine),
                 Print(&self.message), 
                 MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16), 
                 Show   
                ).unwrap();
    }

    pub fn set_message(&mut self, message: String, secs: i64) {

        self.message = message;
        self.message_time = Some((SystemTime::now(), secs));
    }

}