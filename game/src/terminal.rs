use std::io::stdin;

use crate::{server::Server, State};

pub struct Terminal {
    state: State,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            state: State {
                servers: Vec::new(),
                selected: 0,
            },
        }
    }

    pub fn run(&mut self) {
        println!("servers: {:?}", self.state.servers);
        // for line in stdin().lines().map(|x| x.unwrap()) {
        //     let mut words = line.split(' ');
        //     if let Some(command) = words.next() {
        //         match command {
        //             "echo" => {
        //                 println!("echo: {}", words.next().unwrap_or(""));
        //             }
        //             _ => {
        //                 println!("command not found")
        //             }
        //         }
        //     }
        // }
    }
}
