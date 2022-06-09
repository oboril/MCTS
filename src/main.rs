#![allow(dead_code)]

mod tictactoe;
//use tictactoe::TicTacToe;
mod mcts;
use core::panic;

use mcts::{Node, GeneralGame};

mod connect4;
use connect4::Connect4;
use rand::prelude::ThreadRng;

fn play_connect4_against_computer() {
    let mut line = String::new();
    println!("Enter bot accuracy: ");
    std::io::stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace());
    let max_bot_rollouts = line.parse::<usize>().unwrap();
    line = String::new();
    println!("Enter eval accuracy: ");
    std::io::stdin().read_line(&mut line).unwrap();
    line.retain(|c| !c.is_whitespace());
    let max_eval_rollouts = line.parse::<usize>().unwrap();
    println!("Settings: bot:{} eval:{}", max_bot_rollouts, max_eval_rollouts);

    fn index_from_input() -> Option<usize> {
        let mut line = String::new();
    
        std::io::stdin().read_line(&mut line).unwrap();
        
        line.retain(|c| !c.is_whitespace());
    
        let index = line.parse::<usize>();
        if index.is_err() { return None; }
        let index = index.unwrap();
        if index < 1 || index > 6 { return None; }

        return Some(index-1);
    }

    let mut rng = rand::thread_rng();
    

    let mut board = Connect4::empty();

    let mut player = 1i8;

    let mut round = 0usize;
    while board.get_score() == 0 && board.get_available().len() > 0 {
        println!("{}", board);

        // Print evaluation
        {
            let mut node = Node::new(board.clone(), player, 0);
            for _ in 0..max_eval_rollouts {
                node.propagate(1, &mut rng);
            }
            let mut wins = (node.wins as f32)/(node.visits as f32)*100.;
            let mut losses = (node.losses as f32)/(node.visits as f32)*100.;
            if player == 1 {
                (wins, losses) = (losses, wins);
            }
            println!("Human wins: {:0.1}%, Computer wins: {:0.1}%", wins, losses);
            //println!("Best move is {}", node.get_most_visited_child().unwrap().move_index+1);
        }
        
        // Human
        if player == 1 {
            round += 1;
            println!("\u{001b}[32;1mRound {}\u{001b}[0m", round);


            let mut index : Option<usize> = None;
            let available = board.get_available();
            while index.is_none() || !available.contains(&index.unwrap()){
                println!("Select where do you want to place the token (1-6):");
                index = index_from_input();
            }

            board.update(index.unwrap(), player);
        }
        // Computer
        else if player == -1 {
            let mut node = Node::new(board.clone(), player, 0);
            for _ in 0..max_bot_rollouts {
                node.propagate(1, &mut rng);
            }

            let index = node.get_most_visited_child().unwrap().move_index;

            board.update(index, player);
        }

        player *= -1;
    }

    println!("{}", board);
    println!("Game over!");
}

fn simulate_game(bot1_evals: usize, bot2_evals: usize, bot1_rollouts:usize, bot2_rollouts:usize) -> i8 {
    let mut board = Connect4::empty();

    let mut rng = rand::thread_rng();

    let mut player = 1i8;

    while board.get_score() == 0 && board.get_available().len() > 0 {
        let mut node = Node::new(board.clone(), player, 0);
        let max_evals = if player == 1 {bot1_evals} else {bot2_evals};
        let rollouts = if player == 1 {bot1_rollouts} else {bot2_rollouts};

        for _ in 0..max_evals {
            node.propagate(rollouts as u64, &mut rng);
        }

        let best_move = node.get_most_visited_child().unwrap().move_index;
        board.update(best_move, player);

        player *= -1;
    }

    return board.get_score();
}

fn train_neural_net() {
    use neural_nets::*;
    create_nn!(
        MyModel,
        [
            dense1: neural_nets::layers::Dense<72,128>,
            act1: neural_nets::layers::LeakyRelu1D<128>,
            dense2: neural_nets::layers::Dense<128, 128>,
            act2: neural_nets::layers::LeakyRelu1D<128>,
            dense3: neural_nets::layers::Dense<128, 128>,
            act3: neural_nets::layers::LeakyRelu1D<128>,
            dense4: neural_nets::layers::Dense<128, 64>,
            act4: neural_nets::layers::LeakyRelu1D<64>,
            dense5: neural_nets::layers::Dense<64, 32>,
            act5: neural_nets::layers::LeakyRelu1D<32>,
            dense6: neural_nets::layers::Dense<32, 6>,
            act6: neural_nets::layers::Softmax<6>
        ],
        neural_nets::losses::CrossEntropy1D<6>
    );

    fn flip_board(connect4: &mut Connect4) {
        for i in 0..6 {
            for j in 0..6{
                connect4.board[i][j] *= -1;
            }
        }
    }

    fn get_input(board : &Connect4) -> [f32;72]{
        let mut out = [0.0f32; 72];

        for i in 0..6 {
            for j in 0..6 {
                if board.board[i][j] == -1 { out[i*6+j] = 1.; }
                if board.board[i][j] == 1 { out[i*6+j + 36] = 1.; }
            }
        }

        return out;
    }

    let mut root_node = Node::<Connect4>::new(Connect4::empty(), 1, 0);

    let mut model = Box::new(MyModel::new());

    let mut loss = 0.0f32;
    let mut accuracy = 0.0f32;
    let mut groundtruth = [0f32;6];
    for batch in 0..100000 {
        for sample in 0..128 {
            // get current board
            let input = get_input(&root_node.game);

            // reset board if needed
            if root_node.game.get_score() != 0 || root_node.game.get_available().len() == 0 {
                root_node = Node::<Connect4>::new(Connect4::empty(), 1, 0);
            }

            root_node.predict(100, 1);
            
            assert!(root_node.player == 1);
            
            // create ground_truth vector
            groundtruth = [0f32;6];
            let sum = root_node.children.iter().map(|ch| ch.visits).sum::<u64>() as f32;
            for ch in root_node.children.iter() {
                groundtruth[ch.move_index] = (ch.visits as f32) / sum;
            }
            let mut sum = 0f32;
            for i in 0..6{
                //groundtruth[i] = groundtruth[i]*groundtruth[i];
                sum += groundtruth[i];
            }
            assert!(sum > 0.);
            for i in 0..6{
                groundtruth[i] = groundtruth[i]/ sum;
            }

            // get best next move
            if let Some(next) = root_node.get_most_visited_child() {
                root_node = next.clone();
                root_node.player = 1;
                flip_board(&mut root_node.game);
            }
            else {
                println!("{}", root_node.game);
                use std::io::Write;
                std::io::stdout().flush();
                panic!("This should not happen");
            }

            model.feedforward(&input);
            model.backpropagate(&groundtruth);
            model.update_gradient(128., 0.2);

            loss += model.get_loss(&groundtruth);

            let output = model.get_output();
            let mut max_output = 0usize;
            let mut max_groundtruth = 0usize;
            for i in 0..6 {
                if output[i] > output[max_output] { max_output = i; }
                if groundtruth[i] > groundtruth[max_groundtruth] { max_groundtruth = i; }
            }
            if max_groundtruth == max_output {accuracy += 1.; }
        }
        let learning_rate = if batch < 20000 {0.03} else if batch < 40000 {0.01} else if batch < 60000 {0.003} else if batch < 80000 {0.001} else {0.0003};
        model.update_weights(learning_rate, 0.2, 0.0);

        if batch%100 == 0 {
            println!("Batch: {}, Loss: {}, accuracy: {}, gt {:?}", batch, loss/128./100., accuracy/128./100., groundtruth);
            accuracy = 0.0;
            loss = 0.0;
        }
    }
}

fn main() {
    train_neural_net();
}
