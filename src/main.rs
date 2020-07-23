mod editor;
#[path = "editor_src/editor_prompts.rs"] mod editor_prompts;
#[path = "editor_src/render_input.rs"] mod render_input;
#[path = "editor_src/char_row_manipulation.rs"] mod char_row_manipulation;
#[path = "editor_src/bar_and_message.rs"] mod bar_and_message;

mod row;

use editor::{Editor, get_syntax};
use row::Row;

use std::io::{Write, stdout};

use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    execute

};

pub struct Syntax;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    let res: Result<(), Box<dyn std::error::Error>> = {

        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        let mut editor = Editor::new()?;

        if let Some(v) = std::env::args().nth(1) {
            editor.open(v)?;
        }


        editor.set_message(String::from("sair: CTRL-Q | salvar: CTRL-S"), -1);
        editor.main_loop(true);
        loop {
            if let Ok(_) = editor.process_input() {
                editor.main_loop(false);
            }
            else {
                break;
            }
        }

        
        //std::fs::File::create("saida.log")?.write(editor.log.as_bytes()).unwrap();

        Ok(())
    };


    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    match res {
        Err(error) => {
            println!("erro: {:?}", error);
        }
        _ => ()
    }

    

    Ok(())
}