mod tictactoe;
use tictactoe::TicTacToe;
mod mcts;
use mcts::Node;

fn main() {
    let mut rng = rand::thread_rng();
    let mut node = Node::new(TicTacToe::from_string(".O.\n.X.\nX..").unwrap(), -1);

    for _ in 0..100000usize {
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
