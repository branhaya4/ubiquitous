pub mod server;
pub mod terminal;

use server::Server;

pub struct State {
    // will be tree
    pub servers: Vec<Server>,
    pub selected: usize,
}
