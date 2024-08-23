use bilibili_music;
use test_context::TestContext;


struct Login {
    key: String
}

impl TestContext for Login {
    fn setup() -> Login {
        Login {key: "hello".to_string()}
    }
    fn teardown(self) {
        println!("hello,world")
    }
}

#[test]
fn test_login_api() {
    let ok = bilibili_music::api::login::login();
    assert_eq!(true, ok);
}
