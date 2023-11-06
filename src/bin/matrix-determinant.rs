/// [kyu]
/// 4
///
/// [description]
/// Write a function that accepts a square matrix (N x N 2D array)
/// and returns the determinant of the matrix.
/// Find det(M) by using the Gauss-Jordan method.
/// 1. Triangle the matrix using the Gauss pivot method
/// 2. The determinant of the triangle matrix is the product of the diagonal elements
///
/// # Example :
///
/// Considering the following matrix :
/// ```
/// 2  5  3
/// 1 -2 -1
/// 1  3  4
/// ```
/// The corresponding triangle matrix is the following :
/// ```
/// 2  5    3
/// 0 -9/2 -5/2
/// 0  0    20/9
/// ```
///
/// Then the determinant of the matrix is the product of the diagonal values, hence :
///
///     Det(M) = 2 * -9/2 * 20/9 = -20
///
use num::{Rational64, Zero};
use std::cmp::{max, min};

struct Matrix {
    pub mat: Vec<Vec<Rational64>>,
    permutations: Vec<(usize, usize)>,
}

impl From<&[Vec<i64>]> for Matrix {
    fn from(value: &[Vec<i64>]) -> Self {
        let matrix = value
            .iter()
            .map(|row| row.iter().map(|&i| Rational64::from(i)).collect())
            .collect::<Vec<Vec<Rational64>>>();
        Matrix {
            mat: matrix,
            permutations: Vec::new(),
        }
    }
}

impl Matrix {
    pub fn swap_rows(&mut self, ind_a: usize, ind_b: usize) {
        self.mat.swap(ind_a, ind_b);
        self.permutations.push((ind_b, ind_a));
    }
}

/// Given a square matrix, return the corresponding triangle matrix
fn get_triangle_matrix(matrix: &[Vec<i64>]) -> Matrix {
    // here, we know we only deal with square matrices, so we don't check dimensions
    // In a real-world scenario, we should do it
    let dim = matrix.len();
    let mut matrix = Matrix::from(matrix);
    for i in 0..(dim - 1) {
        let pivot_row = matrix
            .mat
            .iter()
            .skip(i)
            .enumerate()
            .find(|row| !row.1[i].is_zero())
            .unwrap();
        if pivot_row.0 != 0 {
            matrix.swap_rows(pivot_row.0 + i, i);
        }
        let pivot_row = matrix.mat[i].clone();
        for j in (i + 1)..dim {
            let mul = matrix.mat[j][i] / pivot_row[i];
            for (k, cell) in pivot_row.iter().enumerate().skip(i) {
                matrix.mat[j][k] -= cell * mul;
            }
        }
    }
    matrix
}

fn determinant(matrix: &[Vec<i64>]) -> i64 {
    match matrix.len() {
        1 => matrix[0][0],
        2 => matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0],
        _ => {
            let matrix = get_triangle_matrix(matrix);
            let det = (0..matrix.mat.len())
                .fold(Rational64::from(1), |acc, index| {
                    acc * matrix.mat[index][index]
                })
                .to_integer();
            let nb_permutations: usize = matrix
                .permutations
                .iter()
                .map(|(i, j)| max(i, j) - min(i, j))
                .sum();
            if nb_permutations % 2 == 0 {
                det
            } else {
                -det
            }
        }
    }
}

fn main() {
    assert_eq!(determinant(&[vec![1]]), 1);
    assert_eq!(determinant(&[vec![1, 3], vec![2, 5]]), -1);
    assert_eq!(
        determinant(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]]),
        -20
    );
}
