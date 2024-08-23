use bilibili_music::api;


fn main() {
    let ok = api::login::login();
    println!("{}", ok)
}
