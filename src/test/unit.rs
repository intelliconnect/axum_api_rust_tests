use crate::{get_sample_user, routes::User};

#[test]
fn test_sample_user() {
    let sample_user = User {
        firstname: "Unit".to_owned(),
        lastname: "Test".to_owned(),
        age: 18,
    };

    assert_eq!(get_sample_user(), sample_user)
}
