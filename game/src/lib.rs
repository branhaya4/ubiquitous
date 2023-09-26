pub mod server;
pub mod terminal;

use petgraph::{Graph, Undirected};
use server::Server;

pub struct State {
    // will be tree
    pub servers: Vec<Graph<Server, (), Undirected>>,
    pub selected: usize,
}
