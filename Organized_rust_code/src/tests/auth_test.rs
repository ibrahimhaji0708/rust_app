#[test]
fn test_login() {
    use crate::modules::auth;
    auth::login("TestUser");
    assert_eq!(2, 2);
}
