use crate::Row;
use crate::Editor;


impl Editor {

    pub fn insert_char(&mut self, ch: char) {

        if self.cursor_y == self.row_vec.len() {
            self.row_vec.push(Row::from(&String::new(), self.syntax))
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
            self.row_vec.insert(self.cursor_y, Row::from(&String::new(), self.syntax));
        }
        else if self.cursor_x == self.row_vec[self.cursor_y].len() {
            self.row_vec.insert(self.cursor_y + 1, Row::from(&String::new(), self.syntax));
        }
        else {
            let value = self.row_vec[self.cursor_y].chars[self.cursor_x..].to_string();
            self.row_vec.insert(self.cursor_y + 1, Row::from(&value, self.syntax));

            self.row_vec[self.cursor_y].chars = self.row_vec[self.cursor_y].chars[..self.cursor_x].to_string();
            self.row_vec[self.cursor_y].render_row();
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.next_render_all = true;
    }




}