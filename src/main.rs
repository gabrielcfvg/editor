mod editor;
mod row;

use editor::Editor;
use row::Row;

use std::io::{Write, stdout};

use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    execute

};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let res: Result<(), Box<dyn std::error::Error>> = {

        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;

        let mut editor = Editor::new()?;

        if let Some(v) = std::env::args().nth(1) {
            editor.open(v)?;
        }


        editor.set_message(String::from("sair: CTRL-Q | salvar: CTRL-S"), -1);
        editor.main_loop();
        loop {
            if let Ok(_) = editor.process_input() {
                editor.main_loop();
            }
            else {
                break;
            }
        }

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