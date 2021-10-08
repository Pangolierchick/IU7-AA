use std::ops;
use std::fmt;

pub struct Matrix<T> {
    pub rows : usize,
    pub col  : usize,
    pub data : Vec<T>
}

impl<T> Matrix<T> {
    pub fn new(r : usize, c : usize) -> Self {
        Self { rows : r, col : c, data: Vec::with_capacity(r * c) }
    }

    pub fn get(&self, i : usize, j : usize) -> &T {
        &self.data[i * self.col + j]
    }

    pub fn set(& mut self, i : usize, j : usize, v : T) {
        self.data[i * self.col + j] = v;
    }

    pub fn push(& mut self, v : T) {
        self.data.push(v);
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.rows {
            let mut sub = String::new();
            for j in 0..self.col {
                sub += &self.get(i, j).to_string();
                sub += " ";
            }

            s += &sub;
            s += "\n";
        }

        write!(f, "{}", s)
    }
}
