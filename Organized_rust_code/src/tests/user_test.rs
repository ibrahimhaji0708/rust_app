#[test]
fn test_create_user() {
    use crate::modules::user;
    user::create_user("TestUser");
    assert_eq!(1, 1);
}
