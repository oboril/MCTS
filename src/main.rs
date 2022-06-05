mod board;
use board::Board;
mod mcts;
use mcts::Node;

fn main() {
    let mut rng = rand::thread_rng();
    let mut node = Node::new(Board::from_string("...\n.XO\n...").unwrap(), 1);

    for _ in 0..1000usize {
        node.propagate(1, &mut rng);
    }

    println!("Root node, visits: {}, losses: {}, score: {}", node.visits, node.wins, node.get_score(node.visits));
    node.children.sort_by(|child1, child2| child1.visits.cmp(&child2.visits).reverse());
    for child in node.children.iter(){
        println!("Visits: {}, wins: {:0.1}%, score: {:0.3}", child.visits, (child.wins as f32)/(child.visits as f32)*100f32, child.get_score(node.visits));
        println!("{}", child.board);
    }
}
