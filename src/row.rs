use std::cmp::min;
use std::rc::Rc;
use crate::Syntax;


pub struct Row {
    
    pub syntax: Option<Rc<Syntax>>,
    pub chars: String,
    pub render: String,
    pub highlight: Vec<u8>
}

impl Row {

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn rlen(&self) -> usize {
        self.render.len()
    }

    pub fn hlen(&self) -> usize {
        return self.highlight.len();
    }

    pub fn update_syntax(&mut self) {

        /*
        self.highlight = vec![];        
        
        if let Some(syntax) = self.editor.syntax {
            // self.highlight = self.render.chars().map(|x| *(*syntax).get(&x).unwrap_or(&0u8)).collect();

            let inte: Option<u8> = None;

            for ch in self.render.chars() {

                if let None = inte {
                    self.highlight.push(*(*syntax).get(&ch).unwrap_or(&0u8));
                }
                else if let Some(ct) = inte {
                    self.highlight.push(ct);
                }
            }
        }
        else {
            self.highlight = vec![0; self.rlen()];
        }

        assert_eq!(self.highlight.len(), self.rlen());
        */

        self.highlight = vec![];
        for ch in self.render.chars() {
            if ch.is_numeric() {
                self.highlight.push(1);
            }
            else {
                self.highlight.push(0);
            }
        }

        assert_eq!(self.rlen(), self.hlen());


    }

    pub fn render_row(&mut self) {

        self.render = String::new();

        for ch in self.chars.chars() {
            if ch == '\t' {
                self.render.push_str(" ".repeat(4).as_str());
            }
            else {
                self.render.push(ch);
            }
        }
        self.update_syntax()
    }

    pub fn from(value: &String, syntax: Option<Rc<Syntax>>) -> Self {
    
        let mut saida = Row {
            syntax,
            chars: value.clone(),
            render: String::new(),
            highlight: vec![]
        }; 

        saida.render_row();

        saida
    }

    pub fn change_syntax(&mut self, syntax: Option<Rc<Syntax>>) {
        self.syntax = syntax;
    }

    pub fn cx_to_rx(&self, idx: usize) -> usize {
        let mut saida: usize = 0;

        for ch in self.chars.chars().take(idx) {
            if ch == '\t' {
                saida += 3 - (saida % 3);
            }
            
            saida += 1;
        }
        saida
    }

    pub fn insert_char(&mut self, mut idx: usize, ch: char) { 
        
        idx = min(idx, self.len());

        self.chars.insert(idx, ch);
        self.render_row();
    }

    pub fn delete_char(&mut self, idx: usize) {
        if idx >= self.len() {
            return;
        }

        self.chars.remove(idx);
        self.render_row();
    }

    pub fn push(&mut self, new: String) {
        self.chars.push_str(new.as_str());
        self.render_row();
    }

}
