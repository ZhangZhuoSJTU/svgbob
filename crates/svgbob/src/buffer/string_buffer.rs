use std::ops::{Deref, DerefMut};
use unicode_width::UnicodeWidthChar;

fn append_line(row: &mut Vec<char>, chars: &mut std::str::Chars<'_>) {
    if let Some(ch) = chars.next() {
        let width = ch.width().unwrap_or(1);
        append_line(row, chars);
        for _ in 1..width {
            row.push('\0');
        }
        row.push(ch);
    }
}

/// A 2D string buffer
/// where you can insert a character to any cell on the 2D grid
/// each cell can be assigned with a string
/// taking into account utf8 code which can not be char
/// including but not limited to multi-width chars
#[derive(Default)]
pub struct StringBuffer(Vec<Vec<char>>);

impl Deref for StringBuffer {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StringBuffer {
    pub fn new() -> Self {
        StringBuffer::default()
    }

    /// add rows to this buffer
    fn add_rows(&mut self, n: i32) {
        for _i in 0..n {
            self.push(vec![]);
        }
    }

    /// add columns to the specified row of this buffer
    fn add_column(&mut self, row: usize, n: i32) {
        for _i in 0..n {
            self[row].push(' ');
        }
    }

    /// put a character in this x and y location, replacing the old character
    /// if there is already
    /// x and y can also be negative
    pub fn add_char(&mut self, x: i32, y: i32, ch: char) {
        if x >= 0 && y >= 0 {
            if ch == '\0' {
                println!("skipping {}", ch);
            } else {
                let row_index = y as usize;
                let column_index = x as usize;
                let row_diff = y - self.len() as i32;
                if row_diff >= 0 {
                    self.add_rows(row_diff + 1);
                }
                let column = &self[y as usize];
                let column_diff = x - column.len() as i32;
                if column_diff >= 0 {
                    self.add_column(row_index as usize, column_diff + 1);
                }
                self[row_index][column_index] = ch;
            }
        }
    }

    /// replace insert a string on this location
    pub fn add_str(&mut self, x: i32, y: i32, s: &str) {
        if y < 0 {
            for (offset, ch) in s.chars().enumerate() {
                self.add_char(x + offset as i32, y, ch);
            }
            return;
        }

        if let Some(max_column) = s
            .chars()
            .enumerate()
            .filter_map(|(offset, ch)| {
                let column = x + offset as i32;
                (column >= 0 && ch != '\0').then_some(column)
            })
            .max()
        {
            let row_index = y as usize;
            let row_diff = y - self.len() as i32;
            if row_diff >= 0 {
                self.add_rows(row_diff + 1);
            }

            let column_diff = max_column - self[row_index].len() as i32;
            if column_diff >= 0 {
                self.add_column(row_index, column_diff + 1);
            }
        }

        let row_index = y as usize;
        for (offset, ch) in s.chars().enumerate() {
            let column = x + offset as i32;
            if column >= 0 {
                if ch == '\0' {
                    println!("skipping {}", ch);
                } else {
                    self[row_index][column as usize] = ch;
                }
            }
        }
    }
}

impl From<&str> for StringBuffer {
    fn from(s: &str) -> Self {
        let mut rows = vec![];
        for line in s.lines() {
            let mut row = vec![];
            append_line(&mut row, &mut line.chars());
            row.reverse();
            rows.push(row);
        }

        StringBuffer(rows)
    }
}

impl ToString for StringBuffer {
    fn to_string(&self) -> String {
        let mut output = String::new();
        for (row_index, row) in self.iter().enumerate() {
            if row_index > 0 {
                output.push('\n');
            }
            let Some(last_content) = row
                .iter()
                .rposition(|ch| *ch != '\0' && !ch.is_whitespace())
            else {
                continue;
            };
            for ch in &row[..=last_content] {
                if *ch != '\0' {
                    output.push(*ch);
                }
            }
        }
        output
    }
}
