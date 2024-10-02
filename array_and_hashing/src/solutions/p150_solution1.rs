use std::collections::HashSet;

//You are given a a 9 x 9 Sudoku board board. A Sudoku board is valid if the following rules are followed:
//
//Each row must contain the digits 1-9 without duplicates.
//Each column must contain the digits 1-9 without duplicates.
//Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without duplicates.
//Return true if the Sudoku board is valid, otherwise return false
//
//Note: A board does not need to be full or be solvable to be valid.
//
//Example 1:
//Input: board =
//[["1","2",".",".","3",".",".",".","."],
// ["4",".",".","5",".",".",".",".","."],
// [".","9","8",".",".",".",".",".","3"],
// ["5",".",".",".","6",".",".",".","4"],
// [".",".",".","8",".","3",".",".","5"],
// ["7",".",".",".","2",".",".",".","6"],
// [".",".",".",".",".",".","2",".","."],
// [".",".",".","4","1","9",".",".","8"],
// [".",".",".",".","8",".",".","7","9"]]
//
//Output: true
//Example 2:
//
//Input: board =
//[['1','2','.','.','3','.','.','.','.'],
// ['4','.','.','5','.','.','.','.','.'],
// ['.','9','1','.','.','.','.','.','3'],
// ['5','.','.','.','6','.','.','.','4'],
// ['.','.','.','8','.','3','.','.','5'],
// ['7','.','.','.','2','.','.','.','6'],
// ['.','.','.','.','.','.','2','.','.'],
// ['.','.','.','4','1','9','.','.','8'],
// ['.','.','.','.','8','.','.','7','9']]
//
//Output: false
//Explanation: There are two 1's in the top-left 3x3 sub-box.
//
//Constraints:
//
//board.length == 9
//board[i].length == 9
//board[i][j] is a digit 1-9 or '.'.
//pub fn valid_nine(cells: Vec<Vec<char>>) -> bool{
//    let mut pattern:Vec<bool> = vec![false, false, false, false, false, false, false, false, false];
//    for cell in 0..9{
//        if cells[cell] == '.' {continue}
//        let v:usize = cells[cell].to_string()
//
//    }
//
//}
//
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut sub_box = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let num = board[i][j];
            if num != '.' {
                let b = (i / 3) * 3 + j / 3;
                if !rows[i].insert(num) || !cols[j].insert(num) || !sub_box[b].insert(num) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::solutions::p150_solution1::is_valid_sudoku;

    #[test]
    fn unit_is_valid_sudoku_neg() {
        let test_board = vec![
            vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
            vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '9', '1', '.', '.', '.', '.', '.', '3'],
            vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
            vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(test_board), false);
    }
}
