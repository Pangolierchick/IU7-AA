#[path = "../src/matrix.rs"]
mod matrix;
use rand::Rng;

fn generate_rnd_matrix(size : usize) -> matrix::Matrix<i32> {
    let mut m : matrix::Matrix<i32> = matrix::Matrix::new_zero(size, size);
    let mut rng = rand::thread_rng();

    for i in 0..size {
        for j in 0..size {
            m[[i, j]] = rng.gen::<i32>() % 1000;
        }
    }

    m
}

fn is_equal<T : Default + std::cmp::PartialEq + std::fmt::Display>(m1 : & matrix::Matrix<T>, m2 : & matrix::Matrix<T>) -> bool {
    if m1.rows != m2.rows || m1.col != m2.col {
        return false;
    }

    for (x, y) in m1.data.iter().zip(m2.data.iter()) {
        if x != y {
            return false;
        }
    }

    true
}

#[test]
fn test_default_1() {
    let m1 = matrix::Matrix::from(2, 2, &[1, 2, 3, 4]);
    let m2 = matrix::Matrix::from(2, 2, &[4, 3, 2, 1]);

    match matrix::Matrix::default_mult(&m1, &m2) {
        Err(e) => { println!("{}", e); assert_eq!(true, false);},
        Ok(v) => {
            let true_res = matrix::Matrix::from(2, 2, &[8, 5, 20, 13]);
            assert_eq!(true, is_equal(&v, &true_res));
        }
    }
}

#[test]
fn test_default_2() {
    let m1 = matrix::Matrix::from(1, 3, &[4, 1, 3]);
    let m2 = matrix::Matrix::from(3, 1, &[2, 7, 1]);

    match matrix::Matrix::default_mult(&m1, &m2) {
        Err(e) => { println!("{}", e); assert_eq!(true, false);},
        Ok(v) => {
            let true_res = matrix::Matrix::from(1, 1, &[18]);
            println!("{}\n{}", v, true_res);
            assert_eq!(true, is_equal(&v, &true_res));
        }
    }
}

#[test]
fn test_vinograd_1() {
    let m1 = matrix::Matrix::from(2, 2, &[1, 2, 3, 4]);
    let m2 = matrix::Matrix::from(2, 2, &[4, 3, 2, 1]);

    match matrix::Matrix::vinograd_mult(&m1, &m2) {
        Err(e) => { println!("{}", e); assert_eq!(true, false);},
        Ok(v) => {
            let true_res = matrix::Matrix::from(2, 2, &[8, 5, 20, 13]);
            assert_eq!(true, is_equal(&v, &true_res));
        }
    }
}

#[test]
fn test_vinograd_non_eq() {
    let m1 = matrix::Matrix::from(3, 1, &[4, 1, 3]);
    let m2 = matrix::Matrix::from(3, 2, &[2, 7, 1, 4, 1, 3]);

    match matrix::Matrix::vinograd_mult(&m1, &m2) {
        Err(_) => assert_eq!(true, true),
        Ok(_) =>  assert_eq!(true, false)
    }
}

#[test]
fn test_vinograd_opt_1() {
    let m1 = matrix::Matrix::from(2, 2, &[1, 2, 3, 4]);
    let m2 = matrix::Matrix::from(2, 2, &[4, 3, 2, 1]);

    match matrix::Matrix::vinograd_opt_mult(&m1, &m2) {
        Err(e) => { println!("{}", e); assert_eq!(true, false);},
        Ok(v) => {
            let true_res = matrix::Matrix::from(2, 2, &[8, 5, 20, 13]);
            assert_eq!(true, is_equal(&v, &true_res));
        }
    }
}

#[test]

fn test_vinograd_rnd() {
    let m1 = generate_rnd_matrix(100);
    let m2 = generate_rnd_matrix(100);

    let canon;

    match matrix::Matrix::default_mult(&m1, &m2) {
        Err(_) => { assert_eq!(true, false); return },
        Ok(m)  => canon = m,
    };

    let test;

    match matrix::Matrix::vinograd_mult(&m1, &m2) {
        Err(_) => { assert_eq!(true, false); return },
        Ok(m)  => test = m,
    };

    assert_eq!(true, is_equal(&canon, &test));
}

#[test]

fn test_vinograd_opt_rnd() {
    let m1 = generate_rnd_matrix(100);
    let m2 = generate_rnd_matrix(100);

    let canon;

    match matrix::Matrix::default_mult(&m1, &m2) {
        Err(_) => { assert_eq!(true, false); return },
        Ok(m)  => canon = m,
    };

    let test;

    match matrix::Matrix::vinograd_opt_mult(&m1, &m2) {
        Err(_) => { assert_eq!(true, false); return },
        Ok(m)  => test = m,
    };

    assert_eq!(true, is_equal(&canon, &test));
}
