/// [kyu]
/// 4
///
/// [description]
/// Given a Sudoku data structure with size NxN, N > 0 and √N == integer,
/// write a method to validate if it has been filled out correctly.
///
/// The data structure is a multi-dimensional Array, i.e:
/// ```
/// [
///   [7,8,4,  1,5,9,  3,2,6],
///   [5,3,9,  6,7,2,  8,4,1],
///   [6,1,2,  4,3,8,  7,5,9],
///
///   [9,2,8,  7,1,5,  4,6,3],
///   [3,5,7,  8,4,6,  1,9,2],
///   [4,6,1,  9,2,3,  5,8,7],
///
///   [8,7,6,  3,9,4,  2,1,5],
///   [2,4,3,  5,6,1,  9,7,8],
///   [1,9,5,  2,8,7,  6,3,4]
/// ]
/// ```
/// # Rules for validation
///
/// - Data structure dimension: NxN where N > 0 and √N == integer
/// - Rows may only contain integers: 1..N (N included)
/// - Columns may only contain integers: 1..N (N included)
/// -'Little squares' (3x3 in example above) may also only contain integers: `1..N` (N included)
use itertools::Itertools;
use num::integer::Roots;

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn values_valid(&self) -> bool {
        let dim = self.data.len() as u32;
        self.data
            .iter()
            .all(|row| row.iter().all(|val| (1..=dim).contains(val)))
    }

    fn rows_valid(&self) -> bool {
        self.data.iter().all(|row| row.iter().all_unique())
    }

    fn cols_valid(&self) -> bool {
        self.data
            .iter()
            .enumerate()
            .all(|(ind, _)| (0..self.data.len()).map(|i| self.data[ind][i]).all_unique())
    }

    fn regions_valid(&self) -> bool {
        let region_size = self.data.len().sqrt();
        (0..(self.data.len())).step_by(region_size).all(|row| {
            (0..self.data.len()).step_by(region_size).all(|col| {
                (row..region_size)
                    .flat_map(|i| (col..region_size).map(move |j| self.data[i][j]))
                    .all_unique()
            })
        })
    }

    fn is_valid(&self) -> bool {
        !self.data[0].is_empty()
            && self.values_valid()
            && self.rows_valid()
            && self.cols_valid()
            && self.regions_valid()
    }
}

fn main() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}
