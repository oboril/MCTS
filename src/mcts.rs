use core::panic;

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

pub trait GeneralGame : Clone {
    fn update(&mut self, index:usize, player:i8);
    fn get_score(&self) -> i8;
    fn get_available(&self) -> Vec<usize>;
    //fn get_available(&self) -> (Vec<usize>, Vec<f32>);
}

#[derive(Debug,PartialEq)]
pub struct Node<T:GeneralGame> {
    pub board: T,
    pub player: i8,
    pub visits: u64,
    pub wins: u64,
    pub children: Vec<Node<T>>,
    created_children: bool
}

impl<T:GeneralGame> Node<T> {
    pub fn new(board : T, player: i8) -> Node<T>{
        return Node {board, player: player, visits: 0, wins: 0, children: Vec::new(), created_children: false};
    }

    pub fn rollout(&self, rng: &mut ThreadRng) -> i8 {
        let mut current_board = self.board.clone();
        let mut current_player = self.player;

        loop {
            let score = current_board.get_score();
            if score != 0 {
                return score;
            }

            let available = current_board.get_available();

            if available.len() == 0 {
                return 0;
            }

            let index = *available.choose(rng).unwrap();
            current_board.update(index, current_player);
            current_player *= -1;
        }
    }

    pub fn create_children(&mut self){
        self.created_children = true;

        // If someone already won, there is no point in creating children
        if self.board.get_score() != 0{
            return;
        }


        let available = self.board.get_available();

        for index in available{
            let mut child = Node::new(self.board.clone(), -self.player);
            child.board.update(index, self.player);
            self.children.push(child);
        }
    }

    pub fn get_score(&self, parent_visits: u64) -> f32 {
        const UPPER_BOUND_CONSTANT : f32 = 1.47;

        if self.visits == 0 {
            return f32::INFINITY;
        }

        let fwins = self.wins as f32;
        let fvisits = self.visits as f32;
        let fparent_visits = parent_visits as f32;

        return (fwins)/(fvisits) + UPPER_BOUND_CONSTANT * (fparent_visits.ln() / fvisits).sqrt();
    }

    pub fn get_child_with_highest_score(&self, rng: &mut ThreadRng) -> Option<usize> {
        if self.children.len() == 0{
            return None;
        }
        let mut max_score = f32::NEG_INFINITY;
        let mut max_index = 0usize;

        for (index, node) in self.children.iter().enumerate(){
            let score = node.get_score(self.visits);
            if score > max_score {
                max_index = index;
                max_score = score;
            }
        }

        // if some nodes were not visited yet, select random
        if max_score == f32::INFINITY {
            let not_visited = self.children.iter().enumerate().filter_map(|(i, n)| if ! n.created_children {Some(i)} else {None});
            return not_visited.choose(rng);
        }

        return Some(max_index);
    }

    pub fn propagate(&mut self, rollouts: u64, rng: &mut ThreadRng) -> (u64, u64){
        // returns (visits, player1 wins, player-1 wins)
        self.visits += rollouts;

        // if someone has already won, just return the winner
        let score = self.board.get_score();
        if score != 0 {
            if score == 1 {
                if self.player == -1 { self.wins += rollouts; }
                return (rollouts, 0);
            }
            else if score == -1 {
                if self.player == 1 { self.wins += rollouts; }
                return (0, rollouts)
            }
            else {
                panic!("Invalid score");
            }
        }

        let (mut wins_1, mut wins_n1) = (0u64, 0u64);

        // If the children have not been created yet, do rollouts and initialize children
        if ! self.created_children {
            self.create_children();

            for _ in 0..rollouts {
                let res = self.rollout(rng);
                
                if res == 1{
                    wins_1 += 1;
                }
                else if res == -1 {
                    wins_n1 += 1;
                }
            }
        }
        // recursively call next children with highest score
        else {
            let next = self.get_child_with_highest_score(rng);
            if let Some(next_node_index) = next {
                (wins_1, wins_n1) = self.children[next_node_index].propagate(rollouts, rng);
            }
        }

        // update self
        if self.player == -1 {
            self.wins += wins_1;
        }
        else if self.player == 1 {
            self.wins += wins_n1;
        }

        return (wins_1, wins_n1);
    }
}


#[cfg(test)]
use super::board::Board;
#[test]
fn test_node_new(){
    let board = Board::from_string("..X\nO..\nXXO").unwrap();
    let node = Node::new(board, -1);

    let board = Board::from_string("..X\nO..\nXXO").unwrap();
    assert_eq!(node, Node {board: board, player: -1, visits: 0, wins: 0, children: Vec::<Node<Board>>::new(), created_children: false})
}

#[test]
fn test_node_rollout(){
    let mut rng = rand::thread_rng();

    let board = Board::from_string("XX.\nOOX\nOXO").unwrap();
    let mut node = Node::new(board, -1);

    assert_eq!(node.rollout(&mut rng), -1);

    node.player = 1;
    assert_eq!(node.rollout(&mut rng), 1);

    let board = Board::from_string("...\n...\n...").unwrap();
    let node = Node::new(board, -1);
    const MAX_ITER:usize = 10000;
    let mut iter = 0usize;
    let (mut player_1, mut player_2, mut draw) = (false, false, false);
    while !(player_1&&player_2&&draw){
        iter += 1;
        let res = node.rollout(&mut rng);
        match res {
            -1 => player_1=true,
            0 => draw = true,
            1 => player_2=true,
            _ => panic!("Invalid result")
        };
        assert!(iter < MAX_ITER);
    }
}

#[test]
fn test_node_create_children(){
    let board = Board::from_string("..X\nO..\nXXO").unwrap();
    let mut node = Node::new(board, -1);

    node.create_children();
    assert_eq!(node.children.len(), 4);
    assert_eq!(node.children[0].board, Board::from_string("O.X\nO..\nXXO").unwrap());
}

#[test]
fn test_node_score(){
    let board = Board::from_string("..X\nO..\nXXO").unwrap();
    let mut node = Node::new(board, -1);

    assert_eq!(node.get_score(1), f32::INFINITY);
    node.visits = 1;
    node.wins = 1;
    assert_eq!(node.get_score(1), 1.);

    node.visits = 2;
    node.wins = 1;
    assert!((node.get_score(2) - 1.3654).abs() < 0.001);
}

#[test]
fn test_node_next_maxscore(){
    let mut rng = rand::thread_rng();

    let board = Board::from_string("X.O\nOXO\nXX.").unwrap();
    let mut node = Node::new(board, -1);

    assert_eq!(node.get_child_with_highest_score(&mut rng), None);

    node.create_children();

    assert_ne!(node.get_child_with_highest_score(&mut rng), None);

    const MAX_ITER :usize = 10;
    let mut iter = 0usize;
    loop {
        iter += 1;
        if node.get_child_with_highest_score(&mut rng).unwrap() != 0{
            break;
        }
        assert!(iter<MAX_ITER);
    }

    node.children[0].wins = 0;
    node.children[0].visits = 1;
    node.children[1].wins = 1;
    node.children[1].visits = 2;
    node.visits=3;

    assert!((node.children[0].get_score(3) - 1.541).abs() < 0.001);
    assert!((node.children[1].get_score(3) - 1.589).abs() < 0.001);

    assert_eq!(node.get_child_with_highest_score(&mut rng), Some(1));

    node.children[1].wins = 10;
    node.children[1].visits = 20;
    node.visits = 21;

    assert_eq!(node.get_child_with_highest_score(&mut rng), Some(0));
}