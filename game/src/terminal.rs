use std::io::stdin;

use crate::{State, server::Server};

pub struct Terminal {
    state: State
}

impl Terminal {
    pub fn new(servers: Vec<Server>) -> Self {
        Terminal { state: State {
            servers,
            selected: 0
        } }
    }

    pub fn run(&mut self) {
        for line in stdin().lines().map(|x| x.unwrap()) {
            let mut words = line.split(' ');
            if let Some(command) = words.next() {
                match command {
                    "echo" => {
                        println!("echo: {}", words.next().unwrap_or(""));
                    },
                    _ => {
                        println!("command not found")
                    }
                }
            }
        }
    }
}