use crate::Editor;

use std::time::SystemTime;

use crossterm::{QueueableCommand,
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

        self.my_stdout.queue(Hide).unwrap()
                 .queue(MoveTo(0, self.number_row as u16)).unwrap()
                 .queue(Clear(ClearType::CurrentLine)).unwrap()
                 .queue(SetForegroundColor(Color::Black)).unwrap()
                 .queue(SetBackgroundColor(Color::White)).unwrap()
                 .queue(Print(saida)).unwrap()
                 .queue(ResetColor).unwrap()
                 .queue(MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16)).unwrap()
                 .queue(Show).unwrap();
    }

    pub fn update_message(&mut self) { 

        if let Some(tm) = self.message_time {
            match tm.0.elapsed() {
                Ok(dur) => {
                    if tm.1 != -1 &&dur.as_secs() > tm.1 as u64{
                        self.set_message(String::from("sair: CTRL-Q | salvar: CTRL-S"), -1)
                    }
                }
                _ => ()
            }
        }

        self.my_stdout.queue(Hide).unwrap()
                 .queue(MoveTo(0, (self.number_row + 1) as u16)).unwrap()
                 .queue(Clear(ClearType::CurrentLine)).unwrap()
                 .queue(Print(&self.message)).unwrap()
                 .queue(MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16)).unwrap()
                 .queue(Show).unwrap();
    }

    pub fn set_message(&mut self, message: String, secs: i64) {

        self.message = message;
        self.message_time = Some((SystemTime::now(), secs));
    }

}