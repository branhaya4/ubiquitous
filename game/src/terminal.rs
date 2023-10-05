use std::io::stdin;

use crate::{
    gen::gen_sector,
    server::{utils, AttackInfo, AttackKind, File, Server},
    State,
};

pub struct Terminal {
    state: State,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            state: State {
                sectors: vec![gen_sector(1.)],
                selected: (0, 0),
                skills: [1.; 4],
            },
        }
    }

    pub fn run(&mut self) {
        let fs = &mut self.state.sectors[0].node_weight_mut(0.into()).unwrap().fs;
        fs.files.push(File::new("bazinga".to_string()));
        for line in stdin().lines().map(|x| x.unwrap()) {
            let mut words = line.split(' ');
            if let Some(command) = words.next() {
                let (sector, server) = self.state.selected;
                let sector = &mut self.state.sectors[sector];
                match command {
                    "hack" => {
                        if let Some(kind) = words.next() {
                            if let Some(name) = words.next() {
                                let attack = match kind {
                                    "pwd" | "password" | "crack" => {
                                        Some(AttackInfo { kind: AttackKind::Password, skill: self.state.skills[0] })
                                    }
                                    "proto" | "protomanip" => {
                                        Some(AttackInfo { kind: AttackKind::ProtoManip, skill: self.state.skills[1] })
                                    }
                                    "impersonation" | "phish" | "phishing" => {
                                        Some(AttackInfo { kind: AttackKind::Impersonation, skill: self.state.skills[2] })
                                    }
                                    "collision" | "hash" | "hashing" => {
                                        Some(AttackInfo { kind: AttackKind::Impersonation, skill: self.state.skills[3] })
                                    }
                                    _ => {
                                        dbg!();
                                        None
                                    }
                                };
                                if let Some(attack) = attack {
                                    if utils::hack(
                                        sector,
                                        server.into(),
                                        attack,
                                        name.to_string(),
                                    ).unwrap() {
                                        println!("hacking successful");
                                    } else {
                                        println!("hacking failed");
                                    }
                                } else {
                                    println!("accepts kinds of attacks are password, ");
                                }
                            }
                        } else {
                            println!("usage: hack <kind> <server_name>");
                        }
                    }
                    "ls" => {
                        for name in utils::ls(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    "lsdev" => {
                        for name in utils::lsdev(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    "lsnet" => {
                        for name in utils::lsnet(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    _ => {
                        println!("command not found")
                    }
                }
            }
        }
    }
}
