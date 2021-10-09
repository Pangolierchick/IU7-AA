pub mod matrix;

extern crate num_traits;

fn main() {
    let m : matrix::Matrix<i32> = matrix::Matrix::from(2, 3, &[1, 2, 3, 3, 2, 1]);

    println!("mat:\n{}", m);
}
