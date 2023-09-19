pub struct ServerSecurity {
    // information to be filled
}

impl ServerSecurity {
    pub fn new() -> Self {
        ServerSecurity {}
    }

    pub fn can_compromise(&self, attack: &AttackInfo) -> bool {
        todo!()
    }

    pub fn can_comprimise_root(&self, attack: &AttackInfo) -> bool {
        todo!()
    }
}

pub struct AttackInfo {
    // to be filled
}