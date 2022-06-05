

pub struct Connect4{
    board: [[i8;6];6]
}

impl Connect4 {

}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..3usize{
            for j in 0..3usize{
                write!(f, "{} ", if self.board[i][j] == 1 {'X'} else if self.board[i][j] == -1 {'O'} else {'.'}).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}