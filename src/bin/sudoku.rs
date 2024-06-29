use num_bigint::BigInt;

fn is_valid(board: &[Vec<char>], row: usize, col: usize, c: char) -> bool {
    for i in 0..9 {
        if board[row][i] == c
            || board[i][col] == c
            || board[row / 3 * 3 + i / 3][col / 3 * 3 + i % 3] == c
        {
            return false;
        }
    }
    true
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for c in '1'..='9' {
                    if is_valid(board, i, j, c) {
                        board[i][j] = c;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[i][j] = '.';
                    }
                }
                return false;
            }
        }
    }
    true
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    // efficiency 9
    // let mut board = vec![
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    // ];
    // efficiency 10
    // let mut board = vec![
    //     vec!['.', '.', '.', '.', '.', '.', '6', '8', '.'],
    //     vec!['.', '.', '.', '.', '7', '3', '.', '.', '9'],
    //     vec!['3', '.', '9', '.', '.', '.', '.', '4', '5'],
    //     vec!['4', '9', '.', '.', '.', '.', '.', '.', '.'],
    //     vec!['8', '.', '3', '.', '5', '.', '9', '.', '2'],
    //     vec!['.', '.', '.', '.', '.', '.', '.', '3', '6'],
    //     vec!['9', '6', '.', '.', '.', '.', '3', '.', '8'],
    //     vec!['7', '.', '.', '6', '8', '.', '.', '.', '.'],
    //     vec!['.', '2', '8', '.', '.', '.', '.', '.', '.'],
    // ];
    // efficiency 11
    let mut board = vec![
        vec!['.', '6', '4', '.', '.', '.', '7', '.', '.'],
        vec!['.', '.', '.', '.', '2', '.', '.', '3', '6'],
        vec!['.', '.', '1', '.', '.', '.', '.', '.', '.'],
        vec!['2', '3', '.', '.', '8', '.', '.', '.', '.'],
        vec!['.', '.', '.', '7', '.', '.', '1', '.', '4'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['9', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
        vec!['.', '.', '.', '4', '.', '.', '.', '.', '.'],
    ];

    if solve_sudoku(&mut board) {
        println!("Sudoku solved:");
        print_board(&board);
        let mut res = BigInt::ZERO;
        for b in board.iter() {
            for &c in b.iter() {
                res *= 9;
                res += c as u8 - b'1';
            }
        }
        println!("Encoded: {}", res);
    } else {
        println!("No solution exists.");
    }
}
