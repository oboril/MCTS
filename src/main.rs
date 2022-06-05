mod tictactoe;
//use tictactoe::TicTacToe;
mod mcts;
use mcts::{Node, GeneralGame};

mod connect4;
use connect4::Connect4;

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
        }
        
        // Human
        if player == 1 {
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

fn main() {

    play_connect4_against_computer();

    return;
    let mut rng = rand::thread_rng();

    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";

    let mut node = Node::new(Connect4::from_string(str).unwrap(), 1, 0);

    for _ in 0..1000usize {
        node.propagate(1, &mut rng);
    }

    println!("Root node, visits: {}, wins/losses: {:0.1}%/{:0.1}%, score: {:0.3}",
        node.visits,
        (node.losses as f32)/(node.visits as f32)*100f32,
        (node.wins as f32)/(node.visits as f32)*100f32,
        node.get_score(node.visits));

    node.children.sort_by(|child1, child2| child1.visits.cmp(&child2.visits).reverse());
    for child in node.children.iter(){
        println!("Visits: {}, wins/losses: {:0.1}%/{:0.1}%, score: {:0.3}",
            child.visits, (child.wins as f32)/(child.visits as f32)*100f32,
            (child.losses as f32)/(child.visits as f32)*100f32,
            child.get_score(node.visits));
        println!("{}", child.game);
    }
}
