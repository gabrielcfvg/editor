use std::cmp::min;
use crossterm::style::Color;
use crate::Syntax;

pub struct Row {
    
    pub editor: *mut crate::Editor,
    pub chars: String,
    pub render: String,
    pub highlight: Vec<Color>,
    pub ends_in_comment: bool
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

    pub fn update_syntax(&mut self, row_idx: usize) {

        unsafe {
            
            if let Some(syntax) = (*self.editor).syntax {

                // caso a row anterior termine dentro de um comment block, essa já iniciará nesse estado
                // let mut in_comment_block: bool = {if (*self.editor).row_vec[row_idx].ends_in_comment { true } else { false }};
                //let mut in_comment: bool = false;
                let mut jump: usize = 0;

                self.highlight = vec![Color::White; self.render.len()];
                for (idx, ch) in self.render.chars().enumerate() {
                    
                    if jump != 0 {
                        jump -= 1;
                        continue;
                    }




                    /*

                    código que faria a identificação e rendenização de comentários multilinha

                    if let Some((bc_init, bc_end)) = syntax.block_comment {
                        if in_comment_block {
                            
                            if 
                                ch == bc_end.chars().nth(0).unwrap()     &&
                                self.render.len() - idx >= bc_end.len()  &&
                                &self.render[idx..(idx + bc_end.len())] == bc_end 
                            
                            {
                                
                                for i in idx..idx+bc_end.len() {
                                    self.highlight[idx+i] = syntax.colors.comment;
                                }
                                
                                jump = bc_end.len();
                                in_comment_block = false;
                                continue;
                            }
                            
                            self.highlight[idx] = syntax.colors.comment;
                            continue;
                        }
                        else {
                            
                            if 
                                ch == bc_init.chars().nth(0).unwrap()     && 
                                self.render.len() - idx >= bc_init.len()  &&
                                &self.render[idx..(idx + bc_init.len())] == bc_init 
                            
                            {  
                                for i in idx..idx+bc_init.len() {
                                    self.highlight[idx+i] = syntax.colors.comment;
                                }
                                
                                jump = bc_init.len();
                                in_comment_block = true;
                                continue;
                            }
                        }
                    }
                    */
                

                    if let Some(c_init) = syntax.line_comment {

                        if 
                           ch == c_init.chars().nth(0).unwrap()      && 
                           self.render.len() - idx >= c_init.len()   && 
                           &self.render[idx..(idx+c_init.len())] == c_init
                           
                        {
 
                            for i in idx..(self.render.len()) {
                                self.highlight[i] = syntax.colors.comment;
                            }
                            break;
                        }
                    }

                    if ch.is_numeric() {
                        self.highlight[idx] = syntax.colors.number;
                        continue;
                    }
                    
                    self.highlight[idx] = Color::White;
                    
                }
            
            
            
            
            
            }
            else {
                self.highlight = vec![Color::White; self.rlen()];
            }
        }
    
        assert_eq!(self.rlen(), self.hlen());
    }

    pub fn render_row(&mut self, row_idx: usize) {

        self.render = String::new();

        for ch in self.chars.chars() {
            if ch == '\t' {
                self.render.push_str(" ".repeat(4).as_str());
            }
            else {
                self.render.push(ch);
            }
        }
        self.update_syntax(row_idx);
    }

    pub fn from(row_idx: usize, value: &String, editor: *mut crate::Editor) -> Self {
    
        let mut saida = Row {
            editor,
            chars: value.clone(),
            render: String::new(),
            highlight: vec![],
            ends_in_comment: false,
        }; 

        saida.render_row(row_idx);

        saida
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

    pub fn insert_char(&mut self, row_idx: usize, mut idx: usize, ch: char) { 
        
        idx = min(idx, self.len());

        self.chars.insert(idx, ch);
        self.render_row(row_idx);
    }

    pub fn delete_char(&mut self,row_idx: usize, idx: usize) {
        if idx >= self.len() {
            return;
        }

        self.chars.remove(idx);
        self.render_row(row_idx);
    }

    pub fn push(&mut self, row_idx: usize, new: String) {
        self.chars.push_str(new.as_str());
        self.render_row(row_idx);
    }

}
