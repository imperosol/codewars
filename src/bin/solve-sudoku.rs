use itertools::Itertools;
use std::ops::Deref;

pub struct Grid([[Option<u8>; 9]; 9]);

#[derive(Clone)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Grid {
    pub fn empty_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|&i| i.1.is_none())
                    .map(|i| i.0)
            })
            .enumerate()
            .filter(|(_, row)| row.clone().next().is_some())
            .flat_map(|(row, cols)| cols.map(move |col| Coord { row, col }))
    }

    #[inline(always)]
    pub fn associated_values<'a>(&'a self, coords: &'a Coord) -> impl Iterator<Item = u8> + 'a {
        let in_row = self.0.get(coords.row).unwrap().iter().filter_map(|i| *i);
        let in_col = self.iter().filter_map(|row| *row.get(coords.col).unwrap());
        let in_subsquare = self.get_subsquare(coords).flatten().filter_map(|i| *i);
        in_row.chain(in_col).chain(in_subsquare).unique()
    }

    fn backtrack(
        &mut self,
        empty_coords: &mut (impl Iterator<Item = Coord> + Clone),
    ) -> Result<Self, ()> {
        let coord = empty_coords.next();
        if coord.is_none() {
            // Exploration complete, solution found.
            return Ok(Grid(self.0));
        }
        let coord = coord.unwrap();
        for choice in self.cell_possible_choices(&coord).unwrap() {
            self.0[coord.row][coord.col] = Some(choice);
            match self.backtrack(&mut empty_coords.clone()) {
                Ok(solution) => {
                    return Ok(solution);
                }
                Err(_) => {
                    self.0[coord.row][coord.col] = None;
                }
            }
        }
        // No solution found. Go back and retry
        Err(())
    }

    pub fn solution(mut self) -> Self {
        let mut empty_coords = self.empty_coords().sorted_by(|i, j| {
            self.nb_cell_possible_choices(i)
                .cmp(&self.nb_cell_possible_choices(j))
        });
        self.backtrack(&mut empty_coords).unwrap()
    }

    #[inline(always)]
    pub fn get_cell(&self, coords: &Coord) -> Option<u8> {
        self.0[coords.row][coords.col]
    }

    pub fn get_subsquare<'a>(
        &'a self,
        coords: &'a Coord,
    ) -> impl Iterator<Item = &[Option<u8>]> + 'a {
        self.iter()
            .skip(coords.row / 3 * 3)
            .take(3)
            .map(|row| &row[coords.col / 3 * 3..coords.col / 3 * 3 + 3])
    }

    #[inline(always)]
    fn cell_possible_choices(&self, coords: &Coord) -> Option<impl Iterator<Item = u8>> {
        if self.get_cell(coords).is_some() {
            return None;
        }
        let mut associated = self.associated_values(coords).sorted().peekable();
        Some((1..=9).filter(move |i| associated.next_if(|j| j == i).is_none()))
    }

    #[inline(always)]
    fn nb_cell_possible_choices(&self, coords: &Coord) -> usize {
        if self.get_cell(coords).is_some() {
            return 0;
        }
        9 - self.associated_values(coords).count()
    }
}

impl Deref for Grid {
    type Target = [[Option<u8>; 9]; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    // I use a code that I had already written for another project.
    // The API is slightly different, so I do a conversion beforehand.
    // It performs a useless memory allocation, but it's easier than to
    // adapt the whole existing code
    let mut grid = [[None; 9]; 9];
    for (row_ind, row) in puzzle.iter().enumerate() {
        for (col_ind, col) in row.iter().enumerate() {
            grid[row_ind][col_ind] = match *col {
                0 => None,
                other => Some(other),
            }
        }
    }
    Grid(grid)
        .solution()
        .iter()
        .enumerate()
        .for_each(|(row_idx, row)| {
            row.iter().enumerate().for_each(|(col_idx, val)| {
                puzzle[row_idx][col_idx] = val.unwrap();
            })
        });
}

fn main() {
    let mut puzzle = [
        [0, 0, 8, 0, 3, 0, 5, 4, 0],
        [3, 0, 0, 4, 0, 7, 9, 0, 0],
        [4, 1, 0, 0, 0, 8, 0, 0, 2],
        [0, 4, 3, 5, 0, 2, 0, 6, 0],
        [5, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 6, 0, 3, 0, 9, 4, 1, 0],
        [1, 0, 0, 8, 0, 0, 0, 2, 7],
        [0, 0, 5, 6, 0, 3, 0, 0, 4],
        [0, 2, 9, 0, 7, 0, 8, 0, 0],
    ];
    let solution = [
        [9, 7, 8, 2, 3, 1, 5, 4, 6],
        [3, 5, 2, 4, 6, 7, 9, 8, 1],
        [4, 1, 6, 9, 5, 8, 3, 7, 2],
        [8, 4, 3, 5, 1, 2, 7, 6, 9],
        [5, 9, 1, 7, 4, 6, 2, 3, 8],
        [2, 6, 7, 3, 8, 9, 4, 1, 5],
        [1, 3, 4, 8, 9, 5, 6, 2, 7],
        [7, 8, 5, 6, 2, 3, 1, 9, 4],
        [6, 2, 9, 1, 7, 4, 8, 5, 3],
    ];

    sudoku(&mut puzzle);
    assert_eq!(puzzle, solution);

    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];
    let solution = [
        [6, 1, 5, 7, 2, 4, 8, 3, 9],
        [4, 8, 7, 3, 9, 5, 1, 6, 2],
        [9, 2, 3, 1, 8, 6, 5, 7, 4],
        [5, 9, 8, 4, 3, 2, 7, 1, 6],
        [1, 3, 6, 8, 7, 9, 2, 4, 5],
        [2, 7, 4, 6, 5, 1, 9, 8, 3],
        [8, 4, 9, 5, 1, 3, 6, 2, 7],
        [7, 6, 2, 9, 4, 8, 3, 5, 1],
        [3, 5, 1, 2, 6, 7, 4, 9, 8],
    ];

    sudoku(&mut puzzle);
    assert_eq!(puzzle, solution);
}
