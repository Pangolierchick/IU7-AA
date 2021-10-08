mod matrix;
use std::convert::TryInto;

fn main() {
    let mut m : matrix::Matrix<i32> = matrix::Matrix::new(5, 5);

    for i in 0..m.rows {
        for j in 0..m.col {
            let v : i32 = (i + j).try_into().unwrap();
            m.push(v);
        }
    }

    println!("mat:\n{}", m);
}
