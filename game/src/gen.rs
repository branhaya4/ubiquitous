use std::{vec, string};

use petgraph::adj::NodeIndex;
use rand::{thread_rng, Rng, seq::SliceRandom};

use crate::{
    server::{Device, FileSystem, SecurityState, Server, ServerSecurity, self},
    Sector,
};

pub fn gen_sector(level: f32) -> Sector {
    let base_req = if level == 1. { 1. } else { level * 5. };
    let mut sector = Sector::default();

    let starting_server = gen_server(base_req, true);

    // build nodes
    sector.add_node(starting_server);

    for layer in 1..=9 {
        let layer = layer as f32;
        let layer_incr = if level == 1. { 4. / 10. } else { 5. / 10. };
        let layer_req = base_req + layer * layer_incr;
        for _ in 0..5 {
            sector.add_node(gen_server(layer_req, false));
        }
    }

    sector.add_node(gen_server(level * 5., true));

    // build edges
    for i in 1..=5 {
        sector.add_edge(0.into(), i.into(), ());
    }

    // connect intra-layer
    for layer in 1..=9 {
        for node in 0..5 {
            for _ in 0..thread_rng().gen_range(3..4) {
                let start = 1 + (layer - 1) * 5 + node;
                let end = 1 + (layer - 1) * 5 + thread_rng().gen_range(0..5);
                if start != end && !sector.contains_edge(start.into(), end.into()) {
                    sector.add_edge(start.into(), end.into(), ());
                }
            }
        }
    }

    // connect inter-layer
    for layer in 1..=8 {
        for node in 0..5 {
            for _ in 0..thread_rng().gen_range(3..4) {
                let start = 1 + (layer - 1) * 5 + node;
                let end_local = thread_rng().gen_range(0..5);
                let end = 1 + layer * 5 + end_local;
                if !sector.contains_edge(start.into(), end.into()) {
                    sector.add_edge(start.into(), end.into(), ());
                }
            }
        }
    }

    for i in 0..5 {
        sector.add_edge((1 + 8 * 5 + i).into(), (1 + 9 * 5).into(), ());
    }

    sector
}

fn gen_server(avg_skill: f32, satellite: bool) -> Server {
    let skill_req = [0; 4].map(|_| gen_skill(avg_skill));
    let skill_req_root = [0; 4].map(|_| gen_skill(avg_skill + 0.5));

    Server {
        name: format!("{} {}", gen_server_name(), thread_rng().gen_range(0..100)),
        fs: FileSystem::default(),
        sec: ServerSecurity {
            state: SecurityState::Secure,
            skill_req,
            skill_req_root,
        },
        devices: if satellite {
            vec![Device::Satellite]
        } else {
            vec![]
        },
    }
}

fn gen_skill(avg_skill: f32) -> f32 {
    thread_rng().gen_range((avg_skill - 1.)..(avg_skill + 1.))
}

pub fn gen_server_name() -> &'static str {
    let imported_server_names = include_str!("../assets/servernames.txt");
    //assert_eq!(server_names,"<content of text file>")
    //println!("{server_names}")
    let server_name_list: Vec<&str> = imported_server_names.split("\n").collect();
    let chosen_name: &str = server_name_list.choose(&mut rand::thread_rng()).unwrap();
    chosen_name
}