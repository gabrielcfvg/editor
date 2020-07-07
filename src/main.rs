
use crossterm::{
    cursor::{MoveTo},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write, BufReader, BufRead};
use std::fs::File;
use std::convert::TryInto;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use std::time::SystemTime;

struct Row {
    chars: String,
    render: String,
}

impl Row {
    
    fn len(&self) -> usize {
        return self.chars.len();
    }
    
    fn rlen(&self) -> usize {
        return self.render.len();
    }

    fn renderrow(&mut self) {

        self.render = String::new();

        for a in self.chars.chars() {
            if a == '\t' {
                self.render.push_str(" ".repeat(TAB_SPACES).as_str());
            }
            else {
                self.render.push(a);
            }
        }  
    }

    fn from(text: String) -> Self {

        let chars = text;
        let mut render = String::new();   

        let mut saida = Row{
                chars: chars,
                render: render,
            };
        
        saida.renderrow();
        saida

    }

    fn cx_to_rx(&self, idx: usize) -> usize {

        let mut saida: usize = 0;
        for a in self.chars.chars().take(idx) {
            if a == '\t' {
                saida += (TAB_SPACES - 1) - (saida % TAB_SPACES);
            }
            saida += 1;
        }
        saida
    }

    fn insertchar(&mut self, mut idx: usize, ch: char) {

        if idx > self.len() {
            idx = self.len();
        }

        self.chars.insert(idx, ch);
        self.renderrow();
    }

    fn deletechar(&mut self, idx: usize) {
        if idx < 0 || idx > self.len() {
            return;
        }
        
        self.chars.remove(idx);
        self.renderrow();
    }

    fn push(&mut self, new: String) {

        self.chars.push_str(new.as_str());
        self.renderrow();
    }


}


struct Editor {
    arquivo: String,
    cursor_x: u16,
    cursor_y: u16,
    cols: u16,
    rows: u16,
    rows_vec: Vec<Row>,
    rowoff: u16,
    coloff: u16,
    log: String,
    render_x: u16,
    message: String,
    message_time: Option<(std::time::SystemTime, i64)>,
    fps: u32,
    modified: bool,
    quit_number: u8,
}

impl Editor {

    fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let (cols, rows) = crossterm::terminal::size().unwrap();
        
        execute!(stdout(), MoveTo(0, 0), Clear(ClearType::All));

        Ok(Editor {
            arquivo: String::new(),
            cols: cols,
            rows: rows-2,
            cursor_x: 0,
            cursor_y: 0,
            rows_vec: vec![],
            rowoff: 0,
            log: String::new(),
            coloff: 0,
            render_x: 0,
            message: String::new(),
            message_time: None,
            fps: 0,
            modified: false,
            quit_number: QUIT_NUMBER,
        })
    }

    fn open(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {

        let lista: Vec<String> = BufReader::new(File::open(&path)?).lines().map(|x| x.unwrap().replace("\r", "").replace("\n", "")).collect();
        let lista: Vec<Row> = lista.iter().map(|x| Row::from(x.clone())).collect();

        self.arquivo = path;
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.rows_vec = lista;
        self.rowoff = 0;
        self.coloff = 0;
        self.render_x = 0;


        Ok(())
    }



    fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        if self.arquivo == String::new() {
            self.arquivo = self.prompt("salvar como: ".to_string());
        }

        let saida = self.to_string();
        File::create(&self.arquivo)?.write_all(saida.as_bytes())?;
        self.setmessage(format!("{} bytes gravados no disco!", saida.as_bytes().len()), 4);
        self.modified = false;
        Ok(())
    }

    fn to_string(&self) -> String {

        let mut saida: String = String::new();

        for row in self.rows_vec.iter() {
            saida.push_str(row.chars.as_str());
            saida.push('\n');
        }

        saida
    }




    fn drawrows(&mut self) {
  
        self.render_x = 0;
        if self.cursor_y < self.rows_vec.len() as u16 {
            self.render_x = self.rows_vec[self.cursor_y as usize].cx_to_rx(self.cursor_x as usize).try_into().unwrap();
        }

        if self.cursor_y < self.rowoff {
            self.rowoff = self.cursor_y;
        }
        else if self.cursor_y >= self.rowoff + self.rows {
            self.rowoff = self.cursor_y - self.rows + 1;
        }

        if self.render_x < self.coloff {
            self.coloff = self.render_x;
        }
        else if self.render_x >= self.coloff + self.cols {
            self.coloff = self.render_x - self.cols + 1;
        }


        print!("\x1b[?25l");
        print!("\x1b[H");

        let mut targrow: u16;
        let logo_len: u16 = LOGO.len() as u16;

        for a in 0..self.rows {

            targrow = a + self.rowoff;

            if targrow >= (self.rows_vec.len() as u16) {

                print!("~");
                
                if a == (self.rows/3) && self.cols > (logo_len+4) && self.rows_vec.len() == 0 {

                    let loc: u16 = (self.cols/2) - (logo_len/2);
                    let loc: String = " ".repeat((loc as usize)-1);
                    print!("{}{}", loc, LOGO);
                }
            }

            else {   
                
                if (self.rows_vec[targrow as usize].rlen() as u16) >= self.coloff {
                    let init = self.coloff as usize;
                    let end = std::cmp::min((self.cols+self.coloff) as usize, self.rows_vec[targrow as usize].rlen());
                    
                    print!("{}", &self.rows_vec[targrow as usize].render[init..end]);
                }
            }
            
            print!("\x1b[K");
            if a < self.rows-1 {
                print!("\r\n");
            }
        }

        print!("\x1b[?25h");
        execute!(stdout(), MoveTo(self.render_x - self.coloff, self.cursor_y - self.rowoff));
        
    }

    fn recvkey(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let evento = read().expect("erro ao ler evento");

        match evento {
            
            Event::Key(keyevent) => {

                match keyevent {

                    KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL} => {
                        
                        if self.modified && self.quit_number > 0 {
                            self.setmessage(format!("Aperte CTRL-Q mais {} vezes para sair!!!", self.quit_number), 3);
                            self.quit_number -= 1;
                            return Ok(());
                        }
                        else {
                            execute!(stdout(), crossterm::cursor::MoveTo(0, 0), crossterm::terminal::Clear(crossterm::terminal::ClearType::All));
                            self.quit_number = QUIT_NUMBER;
                            return Err(Box::from("1"));
                        }
                    },

                    KeyEvent{code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL} => {
                        
                        self.quit_number = QUIT_NUMBER;
                        
                        return self.save();
                    },

                    KeyEvent{code: KeyCode::Up, modifiers: _} |
                    KeyEvent{code: KeyCode::Down, modifiers: _} |
                    KeyEvent{code: KeyCode::Left, modifiers: _} |
                    KeyEvent{code: KeyCode::Right, modifiers: _} |
                    KeyEvent{code: KeyCode::PageDown, modifiers: _} |
                    KeyEvent{code: KeyCode::Home, modifiers: _} |
                    KeyEvent{code: KeyCode::End, modifiers: _} |
                    KeyEvent{code: KeyCode::PageUp, modifiers: _} => {
                        if let Err(err) = self.move_cursor(keyevent) {
                            
                            self.quit_number = QUIT_NUMBER;
                            
                            return Err(err);
                        }
                    },
                    
                    KeyEvent{code: KeyCode::Char(ch), modifiers: _} => {
                        if ch.is_ascii() {
                            self.insertchar(ch);
                        }
                    },

                    KeyEvent{code: KeyCode::Backspace, modifiers: _} => {
                        self.deletechar();
                    }

                    KeyEvent{code: KeyCode::Enter, modifiers: _} => {
                        self.insertline();
                    }

                    KeyEvent{code: KeyCode::Tab, modifiers: _} => {
                        for _ in 0..TAB_SPACES {
                            self.insertchar(' ');
                        }
                    }

                    _ => ()
                }
            },
            
            _ => {}
        }

        self.quit_number = QUIT_NUMBER;
        Ok(())
    }

    fn move_cursor(&mut self, key: crossterm::event::KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        
        match key {

            KeyEvent{code: KeyCode::Up, modifiers: _} => {
                if self.cursor_y != 0{
                    self.cursor_y -= 1;
                }
            }
            KeyEvent{code: KeyCode::Down, modifiers: _} => {
                if self.cursor_y < (self.rows_vec.len()) as u16 {
                    self.cursor_y += 1;
                }
            }
            KeyEvent{code: KeyCode::Left, modifiers: _} => {
                if self.cursor_x != 0{
                    self.cursor_x -= 1;
                }
                else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.rows_vec[self.cursor_y as usize].len() as u16;
                }
            }
            KeyEvent{code: KeyCode::Right, modifiers: _} => {
                if (self.cursor_y as usize) < self.rows_vec.len() {
                    if (self.cursor_x as usize) < self.rows_vec[self.cursor_y as usize].len() {
                        self.cursor_x += 1;
                    }
                    else {
                        self.cursor_y += 1;
                        self.cursor_x = 0;
                    }
                }
            }

            KeyEvent{code: KeyCode::PageDown, modifiers: _} => {
                self.cursor_y = std::cmp::min(self.cursor_y + self.rows-4, self.rows_vec.len() as u16);
            }
            KeyEvent{code: KeyCode::PageUp, modifiers: _} => {
                self.cursor_y = std::cmp::max(self.cursor_y - std::cmp::min(self.rows-4, self.cursor_y), 0);
            }

            KeyEvent{code: KeyCode::End, modifiers: _} => {
                self.cursor_x = self.rows_vec[self.cursor_y as usize].len() as u16;
            }
            KeyEvent{code: KeyCode::Home, modifiers: _} => {
                self.cursor_x = 0;
            }


            _ => ()
            
        }

        if (self.cursor_y as usize) < self.rows_vec.len() {
            if (self.cursor_x as usize) > self.rows_vec[self.cursor_y as usize].len() {
                self.cursor_x = self.rows_vec[self.cursor_y as usize].len() as u16
            }
        }
        else {
            self.cursor_x = 0;
        }
    
        Ok(())
    }

    fn prompt(&mut self, men: String) -> String {

        let mut saida = String::new();

        execute!(stdout(), MoveTo(0, self.rows + 1), Clear(ClearType::CurrentLine), Print(&men));
        execute!(stdout(), MoveTo(men.len() as u16, self.rows + 1));
        
        disable_raw_mode().expect("Erro ao desabilitar modo RAW");
        std::io::stdin().read_line(&mut saida).expect("erro ao ler entrada no Prompt");
        enable_raw_mode().expect("Erro ao iniciar modo RAW");

        saida.trim().to_string()
    }



    fn insertchar(&mut self, ch: char) {
        if self.cursor_y as usize == self.rows_vec.len() {
            self.rows_vec.push(Row::from("".to_string()));
        }

        self.rows_vec[self.cursor_y as usize].insertchar(self.cursor_x as usize, ch);
        self.cursor_x += 1;
        self.modified = true;
    }

    fn deletechar(&mut self) {
        if self.cursor_y == 0 && self.cursor_x == 0{
            return;
        }

        if self.cursor_x == 0 && self.cursor_y as usize == self.rows_vec.len() {
            self.cursor_y -= 1;
            self.cursor_x = self.rows_vec[self.cursor_y as usize].len() as u16;
        }
        
        if self.cursor_x > 0 {
            self.rows_vec[self.cursor_y as usize].deletechar((self.cursor_x - 1) as usize);
            self.cursor_x -= 1;
        }
        else {
            self.cursor_x = self.rows_vec[(self.cursor_y - 1) as usize].len() as u16;
            let valor = self.rows_vec[self.cursor_y as usize].chars.clone();
            self.rows_vec[(self.cursor_y - 1) as usize].push(valor);
            self.deleterow(self.cursor_y as usize);
            self.cursor_y -= 1;
        }
    }

    fn deleterow(&mut self, idx: usize) {
    
        if idx < 0 || idx >= self.rows_vec.len() {
            return;
        }
        
        self.rows_vec.remove(idx);
        self.modified = true;

    }

    fn insertline(&mut self) {

        if self.cursor_x == 0 {
            self.rows_vec.insert(self.cursor_y as usize, Row::from("".to_string()));
        }
        else if (self.cursor_x as usize) < self.rows_vec[self.cursor_y as usize].len() {

            let conv = self.rows_vec[self.cursor_y as usize].chars[(self.cursor_x as usize)..].to_string();
            let mut saida = Row{chars: conv, render: String::new()};
            saida.renderrow();
            self.rows_vec.insert((self.cursor_y + 1) as usize, saida);

	    self.rows_vec[self.cursor_y as usize].chars = self.rows_vec[self.cursor_y as usize].chars[..(self.cursor_x as usize)].to_string();
	    self.rows_vec[self.cursor_y as usize].renderrow();

        }
        else{
            self.rows_vec.insert((self.cursor_y + 1) as usize, Row::from("".to_string()));
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
    }



    fn updatebar(&mut self) {

        let mut saida: String = String::new();
        let mut bf: Vec<String> = vec![];
        let mut rp = self.cols.clone();
        let separador: String;


        bf.push(format!("[{}]", if self.arquivo != "" {self.arquivo.clone()} else {"sem nome".to_string()}));
        bf.push(format!("{} linhas", self.rows_vec.len()));
        bf.push(format!("x: {} | y: {}", self.cursor_y+1, self.render_x));
        bf.push(format!("{} FPS", self.fps));

        if self.modified {
            bf.push(String::from("(modificado)"));
        }


        for item in bf.iter() {
            if saida.len() > 0 {
                saida.push_str(" | ");
            }
            saida.push_str(item.as_str());
        }
        rp -= saida.len() as u16;
        saida.push_str(" ".repeat(rp.into()).as_str());


        /*
        let nome: String = if self.arquivo == "".to_string() {
                        "sem nome".to_string()
                    }
                    else {
                        self.arquivo.clone()
                    };

        let max_size = self.cols.clone();

        let item1 = format!("[{}] - {} linhas - ({}|{})", nome, self.rows_vec.len(), self.cursor_y+1, self.render_x);
        let item2 = format!("{} FPS  ", self.fps);

        saida = format!("{}{}{}", item1, " ".repeat((max_size as usize) - (item1.len() + item2.len())), item2);
        */

        print!("\x1b[?25l");
        execute!(
            stdout(),
            MoveTo(0, self.rows),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(Color::Black),
            SetBackgroundColor(Color::White),
            Print(saida),
            ResetColor,
        );
        print!("\x1b[?25h");

        execute!(stdout(), MoveTo(self.render_x - self.coloff, self.cursor_y - self.rowoff));
    }

    fn updatemessage(&mut self) {

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

        execute!(stdout(), MoveTo(0, self.rows + 1), Clear(ClearType::CurrentLine));

        print!("{}", self.message);

        execute!(stdout(), MoveTo(self.render_x - self.coloff, self.cursor_y - self.rowoff));

    }

    fn setmessage(&mut self, message: String, secs: i64) {

        self.message = message;
        self.message_time = Some((std::time::SystemTime::now(), secs));
    }
    
    fn renderscreen(&mut self) {

        let tm = std::time::SystemTime::now();

        self.drawrows();
        self.updatebar();
        self.updatemessage();
        
        if let Ok(t) = tm.elapsed() {
            self.fps = (1_000_000 / t.as_micros()) as u32;
        }
        

    }

}


static LOGO: &str = "Boar Editor";
static TAB_SPACES: usize = 4;
static QUIT_NUMBER: u8 = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let res: Result<(), Box<dyn std::error::Error>> = {
    
        execute!(stdout(), EnterAlternateScreen).unwrap();
        enable_raw_mode().expect("Erro ao iniciar modo RAW");

        let mut editor = Editor::new().unwrap();

        match std::env::args().nth(1) {
            Some(path) => editor.open(path)?,
            None => ()
        }


        editor.setmessage(String::from("sair: CTRL-Q | salvar: CTRL-S"), -1);
        editor.renderscreen();
        loop {
            if let Ok(_) = editor.recvkey() {
                editor.renderscreen();
            }
            else {
                break;
            }
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        disable_raw_mode().expect("Erro ao desabilitar modo RAW");
        execute!(stdout(), LeaveAlternateScreen);

        File::create("saida.log")?.write(editor.log.as_bytes())?;

        Ok(())

    };

    match res {
        Err(_) => {
            disable_raw_mode().expect("Erro ao desabilitar modo RAW");
            execute!(stdout(), LeaveAlternateScreen);
        },
        _ => ()
    }

    Ok(())
}
