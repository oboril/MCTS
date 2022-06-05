use std::fmt;
use super::mcts::GeneralGame;

#[derive(Debug, Clone, PartialEq)]
pub struct TicTacToe{
    pub board: [[i8; 3]; 3]
}

impl TicTacToe {
    pub fn get_score(&self) -> i8 {
        for target in [-1i8, 1i8]{
            // check rows and columns
            for i in 0..3usize{
                if (0..3usize).all(|j| self.board[i][j]==target) {
                    return target;
                }
                if (0..3usize).all(|j| self.board[j][i]==target) {
                    return target;
                }
            }

            // check diagonals
            if (0..3usize).all(|i| self.board[i][i]==target) {
                return target;
            }
            if (0..3usize).all(|i| self.board[i][2-i]==target) {
                return target;
            }
        }

        return 0;
    }

    pub fn get_available(&self) -> Vec<(usize, usize)> {
        let mut res : Vec<(usize, usize)> = Vec::new();
        for i in 0..3usize{
            for j in 0..3usize{
                if self.board[i][j] == 0{
                    res.push((i,j));
                }
            }
        }

        return res;
    }

    pub fn from_string(val : &str) -> Option<TicTacToe> {
        let mut tictactoe = TicTacToe {board:[[0;3];3]};

        for (i,s) in val.chars().enumerate(){
            if (i+1)%4==0 {
                if s != '\n' && s != '\r'{
                    return None;
                }
            }
            else {
                if s != '.'{
                    let target = match s {
                        'X' => 1i8,
                        'O' => -1i8,
                        _ => return None
                    };
                    let x = i%4;
                    let y = i/4;
                    tictactoe.board[y][x] = target;
                }
            }
        }

        return Some(tictactoe)
    }

    pub fn update(&mut self, indeces : (usize, usize), player: i8) {
        self.board[indeces.0][indeces.1] = player;
    }
}

impl GeneralGame for TicTacToe {
    fn get_score(&self) -> i8 {
        return self.get_score();
    }
    fn update(&mut self, index:usize, player:i8) {
        self.update((index/3, index%3), player);
    }

    fn get_available(&self) -> Vec<usize> {
        return Vec::from_iter(self.get_available().iter().map(|(i,j)| i*3+j));
    }
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

#[test]
fn test_tictactoe_score() {
    let mut tictactoe: TicTacToe;

    tictactoe = TicTacToe {board: [[0,0,0],[0,0,0],[0,0,0]]};
    assert_eq!(tictactoe.get_score(), 0);

    tictactoe = TicTacToe {board: [[1,0,0],[1,0,0],[1,0,0]]};
    assert_eq!(tictactoe.get_score(), 1);

    tictactoe = TicTacToe {board: [[0,-1,0],[0,-1,0],[0,-1,0]]};
    assert_eq!(tictactoe.get_score(), -1);

    tictactoe = TicTacToe {board: [[-1,0,0],[0,-1,0],[0,0,-1]]};
    assert_eq!(tictactoe.get_score(), -1);

    tictactoe = TicTacToe {board: [[-1,0,1],[0,1,0],[1,0,-1]]};
    assert_eq!(tictactoe.get_score(), 1);

    tictactoe = TicTacToe {board: [[-1,0,1],[0,0,1],[1,0,1]]};
    assert_eq!(tictactoe.get_score(), 1);
}

#[test]
fn test_tictactoe_available() {
    let mut tictactoe: TicTacToe;

    tictactoe = TicTacToe {board: [[0,0,0],[0,-1,0],[1,0,-1]]};
    assert_eq!(tictactoe.get_available(), [(0,0),(0,1),(0,2),(1,0),(1,2),(2,1)]);

    tictactoe = TicTacToe {board: [[-1,1,0],[-1,-1,-1],[1,1,0]]};
    assert_eq!(tictactoe.get_available(), [(0,2),(2,2)]);
}

#[test]
fn test_tictactoe_fmt(){
    let tictactoe = TicTacToe {board: [[1,1,-1],[0,0,-1],[1,0,0]]};
    let tictactoe_str = format!("{}", tictactoe);
    assert_eq!(tictactoe_str, "X X O \n. . O \nX . . \n");
}

#[test]
fn test_tictactoe_from_string(){
    let mut tictactoe;

    tictactoe = TicTacToe::from_string("XX.\nO.O\n..X\r");
    assert_eq!(tictactoe, Some(TicTacToe {board: [[1,1,0],[-1,0,-1],[0,0,1]]}));

    tictactoe = TicTacToe::from_string("XX.\rO.O\n...X\n");
    assert_eq!(tictactoe, None);
}