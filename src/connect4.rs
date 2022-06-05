use std::fmt;

use super::mcts::GeneralGame;

#[derive(Debug, Clone, PartialEq)]
pub struct Connect4{
    board: [[i8;6];6]
}

impl Connect4 {
    pub fn from_string(val : &str) -> Option<Connect4> {
        let mut connect4 = Connect4 {board:[[0;6];6]};

        for (i,s) in val.chars().enumerate(){
            if (i+1)%7==0 {
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
                    let x = i%7;
                    let y = i/7;
                    connect4.board[y][x] = target;
                }
            }
        }
        // check for gaps
        for col in 0..6usize{
            let mut gap = false;
            for row in (0..6usize).rev() {
                if connect4.board[row][col] == 0{
                    gap = true;
                }
                if connect4.board[row][col] != 0 && gap {
                    return None;
                }
            }
        }

        return Some(connect4)
    }

    pub fn empty() -> Connect4 {
        return Connect4 { board: [[0;6];6] };
    }
}

impl fmt::Display for Connect4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..6usize{
            for j in 0..6usize{
                write!(f, "{} ", if self.board[i][j] == 1 {'X'} else if self.board[i][j] == -1 {'O'} else {'.'}).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl GeneralGame for Connect4 {
    fn get_score(&self) -> i8 {
        let mut count_1 : u8;
        let mut count_n1 : u8;

        // Rows
        for row in 0..6usize {
            count_1 = 0;
            count_n1 = 0;
            for col in 0..6usize {
                if self.board[row][col] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row][col] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= 4 {return 1;}
                if count_n1 >= 4 {return -1;}
            }
        }

        // Columns
        for col in 0..6usize {
            count_1 = 0;
            count_n1 = 0;
            for row in 0..6usize {
                if self.board[row][col] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row][col] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= 4 {return 1;}
                if count_n1 >= 4 {return -1;}
            }
        }

        // Left diagonals
        for offset in (1-6)..6 {
            count_1 = 0;
            count_n1 = 0;
            for row in (-offset).max(0)..(6-offset).min(6) {
                let col = offset + row;

                if self.board[row as usize][col as usize] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row as usize][col as usize] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= 4 {return 1;}
                if count_n1 >= 4 {return -1;}
            }
        }

        // Left diagonals
        for offset in -6..6 {
            count_1 = 0;
            count_n1 = 0;
            for row in (-offset).max(0)..(6-offset).min(6) {
                let col = 5-(offset + row);

                if self.board[row as usize][col as usize] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row as usize][col as usize] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= 4 {return 1;}
                if count_n1 >= 4 {return -1;}
            }
        }

        return 0;
    }

    fn get_available(&self) -> Vec<usize> {
        return Vec::from_iter( (0..6usize).filter(|&col| self.board[0][col] == 0) );
    }

    fn update(&mut self, index:usize, player:i8) {
        for row in (0..6usize).rev() {
            if self.board[row][index] == 0 {
                self.board[row][index] = player;
                return;
            }
        }

        panic!("Out of range.");
    }
}

#[test]
fn test_connect4_fromstr() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str), Some(Connect4 {board: [[1,0,0,0,0,0],[-1,0,0,0,0,0],[1,0,0,-1,0,0],[-1,0,0,1,0,0],[1,-1,0,1,1,0],[1,1,0,-1,-1,-1]]}));

    let str = "\
                        X..O..\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str), None);

    let str = "\
                        X..O..\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                    ";
    assert_eq!(Connect4::from_string(str), None);
}

#[test]
fn test_connect4_score() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), -1);

    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), 0);

    let str = "\
                        X.....\n\
                        X.....\n\
                        X..O..\n\
                        X..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), 1);

    let str = "\
                        X.....\n\
                        X.....\n\
                        X..O..\n\
                        X..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), 1);

    let str = "\
                        X.....\n\
                        X.....\n\
                        O..O..\n\
                        XO.X..\n\
                        XOOXX.\n\
                        XXOOOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), -1);

    let str = "\
                        X.X...\n\
                        O.XX..\n\
                        X.XOX.\n\
                        O.OXOX\n\
                        XOOXXO\n\
                        OOXOXO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), 1);

    let str = "\
                        X..O..\n\
                        O.OO..\n\
                        XOXO..\n\
                        OXXX..\n\
                        XOOXX.\n\
                        OOOXOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), -1);

    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        OX.X..\n\
                        XOXXX.\n\
                        OOOXOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_score(), 1);
}

#[test]
fn test_connect4_available() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_available(), [1,2,3,4,5]);

    let str = "\
                        X..OX.\n\
                        O..XO.\n\
                        X..OX.\n\
                        O..XX.\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_available(), [1,2,5]);

    let str = "\
                        ......\n\
                        X.....\n\
                        X..O..\n\
                        X..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_available(), [0,1,2,3,4,5]);

    let str = "\
                        XXOXXO\n\
                        XOOOOO\n\
                        XOOOOO\n\
                        XOOXOO\n\
                        XOXXXX\n\
                        XXXOOO\n\
                    ";
    assert_eq!(Connect4::from_string(str).unwrap().get_available(), []);
}

#[test]
fn test_connect4_fmt(){
    let connect4 = Connect4{ board: [[-1,0,0,1,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,1,0,-1,1],[-1,0,1,0,0,-1]]};

    let connect4_str = format!("{}", connect4);
    assert_eq!(connect4_str, "O . . X . . \n. . . . . . \n. . . . . . \n. . . . . . \n. . X . O X \nO . X . . O \n");
}

#[test]
fn test_connect4_update(){
    let mut str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOO.\n\
                    ";
    let connect4 = Connect4::from_string(str).unwrap();

    str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        OX.X..\n\
                        XO.XX.\n\
                        OOOOO.\n\
                    ";
    let mut test = connect4.clone();
    test.update(1, 1);
    assert_eq!(test, Connect4::from_string(str).unwrap());

    str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    test = connect4.clone();
    test.update(5, -1);
    assert_eq!(test, Connect4::from_string(str).unwrap());
}