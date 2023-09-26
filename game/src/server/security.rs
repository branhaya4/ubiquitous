pub struct ServerSecurity {
    req_skill: [u32;4]
}

impl ServerSecurity {
    pub fn new() -> Self {
        ServerSecurity {
            req_skill:[0,0,0,0]
        }
    }

    pub fn can_compromise(&self, attack: &AttackInfo) -> bool {
        use AttackKind::*;
        match attack.kind {
            Password => attack.skill >= self.req_skill[0],
            ProtoManip => attack.skill >= self.req_skill[1],
            Impersonation => attack.skill >= self.req_skill[2],
            Collision => attack.skill >= self.req_skill[3],
        }
    }

    pub fn can_compromise_root(&self, attack: &AttackInfo) -> bool {
        todo!()
    }
}

pub struct AttackInfo {
    pub kind: AttackKind,
    pub skill: u32
}

pub enum AttackKind {
    Password,
    ProtoManip,
    Impersonation,
    Collision
}
