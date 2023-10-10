use petgraph::graph::NodeIndex;

use super::{AttackInfo, SecurityState, Server};
use crate::Sector;

pub fn hack(
    sector: &mut Sector,
    index: NodeIndex,
    attack: AttackInfo,
    name: String,
) -> Option<bool> {
    let mut target_index = None;
    for neighbor_index in sector.neighbors(index) {
        if sector.node_weight(neighbor_index).unwrap().name == name {
            target_index = Some(neighbor_index);
            break;
        }
    }
    target_index.map(|target| {
        let security = &mut sector.node_weight_mut(target).unwrap().sec;
        if security.can_compromise(&attack) {
            security.state = SecurityState::Compromised;
            true
        } else {
            false
        }
    })
}

pub fn ls(sector: &Sector, index: NodeIndex) -> Vec<String> {
    let server = sector.node_weight(index).unwrap();
    let fs = &server.fs;
    let mut names = vec![];
    for file in &fs.files {
        names.push(file.name.clone());
    }
    names
}

pub fn lsdev(sector: &Sector, index: NodeIndex) -> Vec<String> {
    let server = sector.node_weight(index).unwrap();
    let devices = &server.devices;
    let mut names = vec![];
    for dev in devices {
        names.push(dev.lsname().to_string());
    }
    names
}

pub fn lsnet(sector: &Sector, index: NodeIndex) -> Vec<String> {
    let mut names = vec![];
    for node in sector.neighbors(index) {
        let server = sector.node_weight(node).unwrap();
        names.push(server.name.clone());
    }
    names
}

pub fn cat(sector: &Sector, index: NodeIndex, name: &str) -> Option<String> {
    let server = sector.node_weight(index).unwrap();
    let fs = &server.fs;
    for file in &fs.files {
        if file.name == name {
            return Some(file.get_contents().clone());
        }
    }
    None
}
