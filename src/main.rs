use rayon::prelude::*;

/// Sequentially multiply A and B square matrices.
pub fn simple_multiply_a_b(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if b.len() == 0 {
        return vec![];
    } else {
        if b[0].len() == 0 {
            return vec![];
        }
    }

    if a.len() == 0 {
        return vec![];
    } else {
        if a[0].len() == 0 {
            return vec![];
        }
    }


    let n = b.len();

    let mut c = vec![vec![0.0_f64; b[0].len()]; a.len()];

    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];



                println!("{}", c[i][j]);
            }
        }
    }
    c
}







use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
fn random_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut rng = thread_rng();
    let uniform = Uniform::from(-1.0..1.0);
    (0..n)
        .map(|_| (0..n).map(|_| uniform.sample(&mut rng)).collect())
        .collect()
}

fn test_sequential_multiplication() {
    let a = random_matrix(150);
    let b = random_matrix(150);
    simple_multiply_a_b(&a, &b);

}

fn main() {

    test_sequential_multiplication();

    

}