#![allow(dead_code)]
#![allow(unused_variables)]

pub mod layers;
mod neural_network;
pub mod losses;

pub use neural_network::NeuralNetwork;
pub use layers::Layer;
pub use losses::Loss;