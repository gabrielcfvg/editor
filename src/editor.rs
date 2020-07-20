use crate::Row;
use std::time::SystemTime;
use std::io::{stdout, Write, BufReader, BufRead};
use std::fs::File;

use crossterm::{execute, 
    cursor::MoveTo, 
    terminal::{Clear, ClearType},
};

pub struct Editor {
    
    pub file: String,
    pub row_vec: Vec<Row>,
    
    #[allow(dead_code)]
    pub log: String,

    pub number_row: usize,
    pub number_col: usize,

    pub cursor_x: usize,
    pub cursor_y: usize,
    pub row_off: usize,
    pub col_off: usize,
    pub render_x: usize,
    pub next_render_all: bool,
 
    pub message: String,
    pub message_time: Option<(SystemTime, i64)>,
    pub fps: usize,
    pub modified: bool,
    pub quit_number: usize,

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

        self.modified = false;
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
            else {
                self.modified = true;
            }
        }

        Ok(())
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