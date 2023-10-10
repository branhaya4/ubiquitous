use crate::server::*;

#[test]
fn fs_test_1() {
    let mut fs = FileSystem::default();
    fs.files.push(File::new("test.txt".to_string()));
    fs.files[0].set_contents("i am a contents :D".to_string());
    assert_eq!(
        fs.files[0].get_contents(),
        &"i am a contents :D".to_string()
    )
}

#[test]
fn sec_test_1() {
    let sec = ServerSecurity {
        skill_req: [1., 1., 0., 0.],
        skill_req_root: [1., 1., 0., 0.],
        state: SecurityState::Secure,
    };
    assert_eq!(
        sec.can_compromise(&AttackInfo {
            kind: AttackKind::Password,
            skill: 1.5
        }),
        true
    );
    assert_eq!(
        sec.can_compromise(&AttackInfo {
            kind: AttackKind::Password,
            skill: 0.5
        }),
        false
    );
    assert_eq!(
        sec.can_compromise(&AttackInfo {
            kind: AttackKind::Impersonation,
            skill: 1.5
        }),
        true
    );
    assert_eq!(
        sec.can_compromise(&AttackInfo {
            kind: AttackKind::Impersonation,
            skill: 0.5
        }),
        true
    );
}
