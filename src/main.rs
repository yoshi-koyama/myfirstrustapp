#[macro_use]
extern crate rocket;

#[get("/current-time")]
fn index() -> String {
    // 現在日付を返す
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[get("/hello")]
fn hello() -> String {
    "Hello, world!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
