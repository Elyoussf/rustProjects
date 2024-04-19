use std::io::{self, Write};

fn main() {
    // Initialize the board with empty strings
    let mut board = [["_"; 3]; 3];

    // Function to display the board (need to be added within a loop)
    display_board(&board);



    let mut row = String::new();
    let mut col = String::new();

    print!("Enter the row number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut row).expect("Failed to read the row number");
    let r: usize = row.trim().parse().expect("Invalid row number");

    print!("Enter the column number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut col).expect("Failed to read the column number");
    let c: usize = col.trim().parse().expect("Invalid column number");

    // Assume the r, c are valid coordinates that are empty
    let letter = "X"; // Assuming player always inserts X

    for i in 0..3 {
        for j in 0..3 {
            if i == r && j == c {
                board[i][j] = letter;
            }
        }
    }

    // Display the updated board
    display_board(&board);
    
}


fn display_board(board: &[[&str; 3]; 3]) {
    for row in board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}
fn checkwinner(board: &[[&str; 3]; 3])->str{
    for  j in 0..3{
        if board[j][0]==board[j][1] && board[j][1]==board[j][2]{
            return board[j][0];
        }
        if board[0][j]==board[1][j] && board[2][j]==board[2][j]{
            return board[j][0];
        }
    }
    if board[0][0]==board[1][1] && board[2][2]==board[0][0]{
        return board[0][0];
    }
    

}