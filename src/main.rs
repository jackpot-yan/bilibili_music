use bilibili_music::api;


fn main() {
    let ok = api::login::login();
    println!("{}", ok);
    let helol = "hello,world".to_string();
    println!("{}", helol);
}
