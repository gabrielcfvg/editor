use crate::Row;
use std::time::SystemTime;
use std::io::{stdout, Write, BufReader, BufRead};
use std::fs::File;
use std::cmp::{min, max};

use crossterm::{execute, 
    cursor::{MoveTo, Hide, Show}, 
    terminal::{Clear, ClearType},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{SetForegroundColor, SetBackgroundColor, Color, Print, ResetColor}
    
};

pub struct Editor {
    
    file: String,
    row_vec: Vec<Row>,
    
    #[allow(dead_code)]
    pub log: String,

    number_row: usize,
    number_col: usize,

    cursor_x: usize,
    cursor_y: usize,
    row_off: usize,
    col_off: usize,
    render_x: usize,
    next_render_all: bool,

    message: String,
    message_time: Option<(SystemTime, i64)>,
    fps: usize,
    modified: bool,
    quit_number: usize,

}

impl Editor {
    
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let (cols, rows) = crossterm::terminal::size().unwrap();   
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All))?;

        Ok(Editor {
            
            file: String::new(),
            row_vec: vec![],
            log: String::new(),

            number_row: (rows-2) as usize,
            number_col: cols as usize,

            cursor_x: 0,
            cursor_y: 0,
            row_off: 0,
            col_off: 0,
            render_x: 0,
            next_render_all: true,

            message: String::new(),
            message_time: None,
            fps: 0,
            modified: false,
            quit_number: 3,

        })
    }

    pub fn open(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {

        let file = File::open(&path)?;
        let file = BufReader::new(file);
        let file: Vec<String> = file.lines().map(|x| x.unwrap().replace("\r", "").replace("\n", "")).collect();
        let lista: Vec<Row> = file.iter().map(|x| Row::from(&x)).collect();


        self.file = path;
        self.row_vec = lista;
        self.modified = false;

        self.cursor_x = 0;
        self.cursor_y = 0;
        self.row_off = 0;
        self.col_off = 0;
        self.render_x = 0;

        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let mut saida: String = String::new();

        for row in self.row_vec.iter() {
            saida.push_str(row.chars.as_str());
            saida.push('\n');
        }

        if self.file.len() > 0 {
            File::create(&self.file)?.write(saida.as_bytes())?;
        }
        else {
            if let Some(path) = self.prompt(String::from("salvar como")) {
                File::create(path)?.write(saida.as_bytes())?;
            }
        }
        self.modified = false;

        Ok(())
    }



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
                    
                KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL} => {
                
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

    pub fn render_screen(&mut self, force_all: bool) {

        let r = self.row_off;
        let c = self.col_off;

        self.calibrate_positions();

        if force_all == false && r == self.row_off && c == self.col_off {

            let idx = self.cursor_y - self.row_off;

            execute!(stdout(), MoveTo(0, (idx) as u16), Hide).unwrap();

            if self.cursor_y >= self.row_vec.len() {
                print!("~");

            }
            else if self.row_vec[self.cursor_y].rlen() >= self.col_off {
                let init = self.col_off;
                let end = min(self.number_col + self.col_off, self.row_vec[self.cursor_y].rlen());

                print!("{}", &self.row_vec[self.cursor_y].render[init..end]);
            }

            execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();

        }
        else {

            execute!(stdout(), MoveTo(0, 0), Hide).unwrap();

            for idx in self.row_off..(self.row_off + self.number_row) {

                if idx >= self.row_vec.len() {
                    print!("~");

                }
                else if self.row_vec[idx].rlen() >= self.col_off {
                    let init = self.col_off;
                    let end = min(self.number_col + self.col_off, self.row_vec[idx].rlen());

                    print!("{}", &self.row_vec[idx].render[init..end]);
                }

                execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
                print!("\r\n")
                
            }
        }

        execute!(stdout(), Show, MoveTo((self.render_x - self.col_off) as u16, (self.cursor_y - self.row_off) as u16)).unwrap();

    }

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


    pub fn insert_char(&mut self, ch: char) {

        if self.cursor_y == self.row_vec.len() {
            self.row_vec.push(Row::from(&String::new()))
        }

        self.row_vec[self.cursor_y].insert_char(self.cursor_x, ch);
        self.cursor_x += 1;
        self.modified = true;
    }

    pub fn delete_char(&mut self) {

        if self.cursor_x == 0 && self.cursor_y == 0 {
            return;
        }

        if self.cursor_x == 0 && self.cursor_y == self.row_vec.len() {
            self.cursor_y -= 1;
            self.cursor_x = self.row_vec[self.cursor_y].len();
        }
        else if self.cursor_x > 0 {
            self.row_vec[self.cursor_y].delete_char(self.cursor_x - 1);
            self.cursor_x -= 1;
        }
        else {
            let valor = self.row_vec[self.cursor_y].chars.clone();
            self.cursor_x = self.row_vec[self.cursor_y-1].len();
            self.row_vec[self.cursor_y -1].push(valor);
            self.delete_row(self.cursor_y);
            self.cursor_y -= 1;
        }

    }

    pub fn delete_row(&mut self, idx: usize) {
    
        if idx >= self.row_vec.len() {
            return;
        }
        
        self.row_vec.remove(idx);
        self.modified = true;
        self.next_render_all = true;
    }

    pub fn new_row(&mut self) {

        if self.cursor_x == 0 {
            self.row_vec.insert(self.cursor_y, Row::from(&String::new()));
        }
        else if self.cursor_x == self.row_vec[self.cursor_y].len() {
            self.row_vec.insert(self.cursor_y + 1, Row::from(&String::new()));
        }
        else {
            let value = self.row_vec[self.cursor_y].chars[self.cursor_x..].to_string();
            self.row_vec.insert(self.cursor_y + 1, Row::from(&value));

            self.row_vec[self.cursor_y].chars = self.row_vec[self.cursor_y].chars[..self.cursor_x].to_string();
            self.row_vec[self.cursor_y].render_row();
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.next_render_all = true;
    }



    fn update_bar(&mut self) {

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

    fn update_message(&mut self) {  

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



    pub fn main_loop(&mut self, force_all: bool) {

        let tm = SystemTime::now();

        self.render_screen(force_all || self.next_render_all);
        self.update_bar();
        self.update_message();
        self.next_render_all = false;

        if let Ok(t) = tm.elapsed() {
            self.fps = (1_000_000 / t.as_micros()) as usize;
        }
    }
}