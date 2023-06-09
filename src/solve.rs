type Grid = [Row; 9];
type Row = [u8; 9];

const GRID_SIZE: usize = 9;

struct Sudoku {
    grid: Grid,
    subgrid: Grid,
    row_lookup: Grid,
    col_lookup: Grid,
    num_solved: u8,
}

impl Sudoku {
    fn new(board: [[u8; 9]; 9]) -> Sudoku {
        // Initalise sudoku struct, including lookup tables for valid moves
        let mut sudoku = Sudoku {
            grid: board,
            subgrid: [[0; 9]; 9],
            row_lookup: [[0; 9]; 9],
            col_lookup: [[0; 9]; 9],
            num_solved: 0,
        };

        // Populate lookup tables with initial values
        for (row_index, row) in board.iter().enumerate() {
            for (col_index, value) in row.iter().enumerate() {
                if *value == 0 {
                    continue;
                }
                sudoku.subgrid[Sudoku::subgrid_index(row_index, col_index)][*value as usize - 1] = 1;
                sudoku.row_lookup[row_index][*value as usize - 1] = 1;
                sudoku.col_lookup[col_index][*value as usize - 1] = 1;
                sudoku.num_solved += 1;
            }
        }
        sudoku
    }

    fn add_value(&mut self, value: &u8, point: &Point) {
        self.grid[point.row][point.col] = *value;
        self.subgrid[Sudoku::subgrid_index(point.row, point.col)][*value as usize - 1] = 1;
        self.row_lookup[point.row][*value as usize - 1] = 1;
        self.col_lookup[point.col][*value as usize - 1] = 1;
        self.num_solved += 1;
    }

    fn remove_value(&mut self, point: &Point) {
        let value = self.grid[point.row][point.col];
        self.grid[point.row][point.col] = 0;
        self.subgrid[Sudoku::subgrid_index(point.row, point.col)][value as usize - 1] = 0;
        self.row_lookup[point.row][value as usize - 1] = 0;
        self.col_lookup[point.col][value as usize - 1] = 0;
        self.num_solved -= 1;
    }

    fn valid_placement(&self, value: &u8, point: &Point) -> bool {
        !(self.row_lookup[point.row][*value as usize - 1] == 1
            || self.col_lookup[point.col][*value as usize - 1] == 1
            || !self.unique_in_subgrid(*value, point))
    }

    fn unique_in_subgrid(&self, value: u8, point: &Point) -> bool {
        let square = Sudoku::subgrid_index(point.row, point.col);
        if self.subgrid[square][value as usize - 1] == 0 {
            return true;
        }
        false
    }

    // Calculate subgrid index based on row and column indexes
    fn subgrid_index(row: usize, col: usize) -> usize {
        (row / 3) * 3 + col / 3
    }
}

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }

    fn next(&self) -> Option<Point> {
        if self.row == GRID_SIZE - 1 && self.col == GRID_SIZE - 1 {
            return None;
        }
        if self.col == GRID_SIZE - 1 {
            Some(Point::new(self.row + 1, 0))
        } else {
            Some(Point::new(self.row, self.col + 1))
        }
    }
}

/// Top level function to solve a sudoku board
pub fn solve(board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    let mut sudoku = Sudoku::new(board);

    let solved = backtrack(&mut sudoku, &Point { row: 0, col: 0 });

    if !solved {
        panic!("No solution found!");
    }

    sudoku.grid
}

fn backtrack(sudoku: &mut Sudoku, point: &Point) -> bool {
    if sudoku.grid[point.row][point.col] != 0 {
        if let Some(next_point) = point.next() {
            return backtrack(sudoku, &next_point);
        } else {
            return true;
        }
    }

    for value in 1..=GRID_SIZE {
        if sudoku.valid_placement(&(value as u8), &point) {
            sudoku.add_value(&(value as u8), &point);
            if let Some(next_point) = point.next() {
                if backtrack(sudoku, &next_point) {
                    return true;
                } else {
                    sudoku.remove_value(&point);
                }
            } else {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_board() {
        assert_eq!(
            solve([
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9],
            ]),
            [
                [5, 3, 4, 6, 7, 8, 9, 1, 2],
                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                [7, 1, 3, 9, 2, 4, 8, 5, 6],
                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                [3, 4, 5, 2, 8, 6, 1, 7, 9],
            ]
        );
    }

    #[test]
    fn test_hard_board() {
        assert_eq!(
            solve([
                [0, 0, 2, 0, 0, 0, 5, 0, 0], 
                [0, 1, 0, 7, 0, 5, 0, 2, 0], 
                [4, 0, 0, 0, 9, 0, 0, 0, 7], 
                [0, 4, 9, 0, 0, 0, 7, 3, 0], 
                [8, 0, 1, 0, 3, 0, 4, 0, 9], 
                [0, 3, 6, 0, 0, 0, 2, 1, 0], 
                [2, 0, 0, 0, 8, 0, 0, 0, 4], 
                [0, 8, 0, 9, 0, 2, 0, 6, 0], 
                [0, 0, 7, 0, 0, 0, 8, 0, 0], 
            ]),
            [
                [9, 7, 2, 8, 6, 3, 5, 4, 1],
                [6, 1, 8, 7, 4, 5, 9, 2, 3],
                [4, 5, 3, 2, 9, 1, 6, 8, 7],
                [5, 4, 9, 1, 2, 8, 7, 3, 6],
                [8, 2, 1, 6, 3, 7, 4, 5, 9],
                [7, 3, 6, 4, 5, 9, 2, 1, 8],
                [2, 9, 5, 3, 8, 6, 1, 7, 4],
                [1, 8, 4, 9, 7, 2, 3, 6, 5],
                [3, 6, 7, 5, 1, 4, 8, 9, 2],
            ]
        );
    }
}
