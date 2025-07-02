use std::collections::HashSet;
type CellType = Cell;
type BoardType = [[CellType; 9]; 9];

#[derive(Debug)]
struct Cell {
    current_value: usize,
    possible_values: Vec<usize>,
}
impl Cell {
    fn new(self) -> CellType {
        Default::default()
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            current_value: self.current_value,
            possible_values: self.possible_values.clone(),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.current_value == other.current_value
    }
}

impl PartialEq<usize> for Cell {
    fn eq(&self, other: &usize) -> bool {
        self.current_value == *other
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            current_value: 0,
            possible_values: initial_possible_values().to_vec(),
        }
    }
}
impl std::ops::Deref for Cell {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.current_value
    }
}
fn slice_board(
    board: &BoardType,
    starting_row: usize,
    starting_column: usize,
    width: usize,
    height: usize,
) -> Vec<&CellType> {
    board[starting_row..height + starting_row]
        .iter()
        .map(|row| &row[starting_column..width + starting_column])
        .flatten()
        .collect()
}

fn get_row(board: &BoardType, row: usize) -> Vec<&CellType> {
    slice_board(&board, row, 0, 9, 1)
}

fn get_column(board: &BoardType, column: usize) -> Vec<&CellType> {
    slice_board(&board, 0, column, 1, 9)
}

fn get_3x3(board: &BoardType, start_y: usize, start_x: usize) -> Vec<&CellType> {
    slice_board(&board, start_y, start_x, 3, 3)
}

fn get_cell(board: &BoardType, cell_y: usize, cell_x: usize) -> Vec<&CellType> {
    get_3x3(&board, cell_y * 3, cell_x * 3)
}
fn print_board(board: &BoardType) {
    for i in 0..9 {
        if i % 3 == 0 {
            println!(" {}", "─".repeat(29));
        }

        for j in 0..9 {
            if j % 3 == 0 {
                print!(" ││");
            }
            print!(" {}", board[i][j].current_value);
        }
        print!(" ││");
        println!();
    }
    println!(" {}", "─".repeat(28));
}

fn check_uniqueness(numbers: &Vec<&CellType>) -> bool {
    let mut set: HashSet<&usize> = Default::default();
    !numbers
        .into_iter()
        .map(move |x| set.insert(x))
        .any(move |x1| x1 == false)
}

fn check_rows_and_cols(board: &BoardType) -> bool {
    let dummy_array = 0_usize..9_usize;
    !dummy_array
        .map(|x| check_uniqueness(&get_row(&board, x)) && check_uniqueness(&get_column(&board, x)))
        .any(|x1| x1 == false)
}

fn check_cells(board: &BoardType) -> bool {
    let mut result = true;
    'outer: for i in 0_usize..3_usize {
        for j in 0_usize..3_usize {
            if !check_uniqueness(&get_cell(&board, i, j)) {
                result = false;
                break 'outer;
            }
        }
    }
    result
}

fn check_board(board: &BoardType) -> bool {
    check_rows_and_cols(&board) && check_cells(&board)
}

fn initial_possible_values() -> [usize; 9] {
    [
        1_usize, 2_usize, 3_usize, 4_usize, 5_usize, 6_usize, 7_usize, 8_usize, 9_usize,
    ]
}

fn example_board() -> BoardType {
    let matrix = [
        [7, 9, 2, 1, 5, 4, 3, 8, 6],
        [6, 4, 3, 8, 2, 7, 1, 5, 9],
        [8, 5, 1, 3, 9, 6, 7, 2, 4],
        [2, 6, 5, 9, 7, 3, 8, 4, 1],
        [4, 8, 9, 5, 6, 1, 2, 7, 3],
        [3, 1, 7, 4, 8, 2, 9, 6, 5],
        [1, 3, 6, 7, 4, 8, 5, 9, 2],
        [9, 7, 4, 2, 1, 5, 6, 3, 8],
        [5, 2, 8, 6, 3, 9, 4, 1, 7],
    ];

    let mut board = BoardType::default();
    for (value, cell) in matrix.as_flattened().iter().zip(board.iter_mut().flatten()) {
        cell.current_value = *value;
    }
    board
}

fn main() {


    let board = example_board();

    print_board(&board);

    println!("Correctness: {}", check_board(&board))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_board_0th_row() {
        let board = example_board();
        let result: Vec<CellType> = slice_board(&board, 0, 0, 9, 1)
            .iter()
            .cloned()
            .cloned()
            .collect();

        assert_eq!(result, board[0]);
    }

    #[test]
    fn test_slice_board_8th_row() {
        let board = example_board();
        let result: Vec<CellType> = slice_board(&board, 8, 0, 9, 1)
            .iter()
            .cloned()
            .cloned()
            .collect();

        assert_eq!(result, board[8]);
    }

    #[test]
    fn test_slice_board_0th_column() {
        let board = example_board();
        let result: Vec<CellType> = slice_board(&board, 0, 0, 1, 9)
            .iter()
            .cloned()
            .cloned()
            .collect();
        let expected: Vec<usize> = vec![7, 6, 8, 2, 4, 3, 1, 9, 5];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_slice_board_8th_column() {
        let board = example_board();
        let result: Vec<CellType> = slice_board(&board, 0, 8, 1, 9)
            .iter()
            .cloned()
            .cloned()
            .collect();
        let expected: Vec<usize> = vec![6, 9, 4, 1, 3, 5, 2, 8, 7];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_slice_board_2x2() {
        let board = example_board();
        let result: Vec<CellType> = slice_board(&board, 1, 1, 2, 2)
            .iter()
            .cloned()
            .cloned()
            .collect();
        let expected: Vec<usize> = vec![4, 3, 5, 1];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_row_0th_row() {
        let board = example_board();
        let result: Vec<CellType> = get_row(&board, 0).iter().cloned().cloned().collect();

        assert_eq!(result, board[0]);
    }

    #[test]
    fn test_get_column_0th_column() {
        let board = example_board();
        let result: Vec<CellType> = get_column(&board, 0).iter().cloned().cloned().collect();
        let expected: Vec<usize> = vec![7, 6, 8, 2, 4, 3, 1, 9, 5];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_3x3_1_1() {
        let board = example_board();
        let result: Vec<CellType> = get_3x3(&board, 1, 1).iter().cloned().cloned().collect();
        let expected: Vec<usize> = vec![4, 3, 8, 5, 1, 3, 6, 5, 9];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_cell_0_0() {
        let board = example_board();
        let result: Vec<CellType> = get_3x3(&board, 0, 0).iter().cloned().cloned().collect();
        let expected: Vec<usize> = vec![7, 9, 2, 6, 4, 3, 8, 5, 1];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_cell_1_1() {
        let board = example_board();
        let result: Vec<CellType> = get_cell(&board, 1, 1).iter().cloned().cloned().collect();
        let expected: Vec<usize> = vec![9, 7, 3, 5, 6, 1, 4, 8, 2];

        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_check_uniqueness() {
    //     let input_vector: Vec<usize> = vec![9, 7, 3, 5, 6, 1, 4, 8, 2];
    //     check_uniqueness(&input_vector.to_vec());
    // 
    //     assert!(check_uniqueness(&input_vector[..]));
    // }
}
