use std::cmp::min;


pub struct Row {
    pub chars: String,
    pub render: String,
}

impl Row {

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn rlen(&self) -> usize {
        self.render.len()
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
    }

    pub fn from(value: &String) -> Self {
    
        let mut saida = Row {
            chars: value.clone(),
            render: String::new()
        };

        saida.render_row();

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
