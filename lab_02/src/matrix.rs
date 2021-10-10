use std::fmt;
use std::ops;
pub struct Matrix<T : Default> {
    pub rows : usize,
    pub col  : usize,
    pub data : Vec<T>
}

impl<T : Default + num_traits::Num + ops::AddAssign + Copy + ops::Neg + ops::SubAssign + ops::Neg<Output = T>> Matrix<T> {
    pub fn new(r : usize, c : usize) -> Self {
        Self { rows : r, col : c, data: Vec::with_capacity(r * c) }
    }

    pub fn new_zero(r : usize, c : usize) -> Self {
        let data : Vec<T> = vec![Default::default(); r * c];

        Self { rows : r, col : c, data }
    }

    pub fn from(r : usize, c : usize, arr : &[T]) -> Self {
        let mut out : Matrix<T> = Matrix::new_zero(r, c);

        for i in 0..r {
            for j in 0..c {
                out[[i, j]] = arr[i * c + j];
            }
        }

        out
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

    pub fn default_mult(m1 : & Matrix<T>, m2 : & Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if m1.col != m2.rows {
            return Err("The number of columns must be equal to number of rows");
        }

        let mut out = Matrix::new_zero(m1.rows, m2.col);
    
        for i in 0..m1.rows {
            for j in 0..m2.col {
                for k in 0..m1.col {
                    out[[i, j]] += m1[[i, k]] * m2[[k, j]];
                }
            }
        }
    
        Ok(out)
    }

    pub fn vinograd_mult(m1 : & Matrix<T>, m2 : & Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if m1.col != m2.rows {
            return Err("The number of columns must be equal to number of rows");
        }

        let mut out : Matrix<T> = Matrix::new_zero(m1.rows, m2.col);

        let mut row_factor : Vec<T> = vec![Default::default(); m1.rows];
        let mut col_factor : Vec<T> = vec![Default::default(); m2.col];

        for i in 0..m1.rows {
            for j in 0..m1.col / 2 {
                row_factor[i] += m1[[i, j * 2]] * m1[[i, j * 2 + 1]]
            }
        }

        for i in 0..m2.col {
            for j in 0..m2.rows / 2 {
                col_factor[i] += m2[[j * 2, i]] * m2[[j * 2 + 1, i]]
            }
        }

        for i in 0..m1.rows {
            for j in 0..m2.col {
                out[[i, j]] -= row_factor[i] + col_factor[j];

                for k in 0..m1.col / 2 {
                    out[[i, j]] += (m1[[i, 2 * k + 1]] + m2[[2 * k, j]]) * (m1[[i, 2 * k]] + m2[[2 * k + 1, j]]);
                }
            }
        }

        if m1.col % 2 > 0{
            for i in 0..m1.rows {
                for j in 0..m2.col {
                    out[[i, j]] += m1[[i, m1.col - 1]] * m2[[m1.col - 1, j]];
                }
            }
        }

        Ok(out)
    }

    pub fn vinograd_opt_mult(m1 : & Matrix<T>, m2 : & Matrix<T>) -> Result<Matrix<T>, &'static str> {
        if m1.col != m2.rows {
            return Err("The number of columns must be equal to number of rows");
        }

        let mut out : Matrix<T> = Matrix::new_zero(m1.rows, m2.col);

        let mut row_factor : Vec<T> = vec![Default::default(); m1.rows];
        let mut col_factor : Vec<T> = vec![Default::default(); m2.col];

        let m1_col_mod = m1.col % 2;
        let m2_row_mod = m2.rows % 2;

        for i in 0..m1.rows {
            for j in (0..m1.col - m1_col_mod).step_by(2) {
                row_factor[i] += m1[[i, j]] * m1[[i, j + 1]]
            }
        }

        for i in 0..m2.col {
            for j in (0..m2.rows - m2_row_mod).step_by(2) {
                col_factor[i] += m2[[j, i]] * m2[[j + 1, i]]
            }
        }

        for i in 0..m1.rows {
            for j in 0..m2.col {
                let mut buf : T = -(row_factor[i] + col_factor[j]);

                for k in (0..m1.col - m1_col_mod).step_by(2) {
                    buf += (m1[[i, k + 1]] + m2[[k, j]]) * (m1[[i, k]] + m2[[k + 1, j]]);
                }

                out[[i, j]] = buf;
            }
        }

        if m1.col % 2 > 0 {
            let m1c = m1.col - 1;
            for i in 0..m1.rows {
                for j in 0..m2.col {
                    out[[i, j]] += m1[[i, m1c]] * m2[[m1c, j]];
                }
            }
        }

        Ok(out)
    }
}

impl<T> fmt::Display for Matrix<T> where 
T: fmt::Display + Default + num_traits::Num + ops::AddAssign + Copy + ops::Neg + ops::SubAssign + ops::Neg<Output = T> {
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

impl<T : Default> ops::Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, index : [usize; 2]) -> &Self::Output {
        &self.data[index[0] * self.col + index[1]]
    }
}

impl<T : Default> ops::IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, index : [usize; 2]) -> &mut Self::Output {
        & mut self.data[index[0] * self.col + index[1]]
    }
} 
