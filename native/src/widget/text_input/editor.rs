use crate::text_input::{Cursor, Value};

pub struct Editor<'a> {
    value: &'a mut Value,
    cursor: &'a mut Cursor,
}

impl<'a> Editor<'a> {
    pub fn new(value: &'a mut Value, cursor: &'a mut Cursor) -> Editor<'a> {
        Editor { value, cursor }
    }

    pub fn contents(&self) -> String {
        self.value.to_string()
    }

    pub fn insert(&mut self, character: char) {
        match self.cursor.selection() {
            Some((left, right)) => {
                self.value.remove_many(left, right);
                self.cursor.move_left(&self.value);
            }
            _ => (),
        }

        self.value.insert(self.cursor.end(&self.value), character);
        self.cursor.move_right(&self.value);
    }

    pub fn paste(&mut self, content: Value) {
        let length = content.len();

        match self.cursor.selection() {
            Some((left, right)) => {
                self.value.remove_many(left, right);
                self.cursor.move_left(&self.value);
            }
            _ => (),
        }

        self.value
            .insert_many(self.cursor.end(&self.value), content);

        self.cursor.move_right_by_amount(&self.value, length);
    }

    pub fn backspace(&mut self) {
        match self.cursor.selection() {
            Some((start, end)) => {
                self.value.remove_many(start, end);
                self.cursor.move_left(&self.value);
            }
            None => {
                let start = self.cursor.start(&self.value);

                if start > 0 {
                    self.cursor.move_left(&self.value);

                    let _ = self.value.remove(start - 1);
                }
            }
        }
    }

    pub fn delete(&mut self) {
        match self.cursor.selection() {
            Some((start, end)) => {
                self.value.remove_many(start, end);
                self.cursor.move_left(&self.value);
            }
            None => {
                let end = self.cursor.end(&self.value);

                if end < self.value.len() {
                    let _ = self.value.remove(end);
                }
            }
        }
    }
}
