#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[get("/current-time")]
fn current_time() -> Json<Response> {
    let now = chrono::Local::now();
    Json(Response {
        message: now.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[get("/hello")]
fn hello() -> Json<Response> {
    Json(Response {
        message: "Hello, world!".to_string(),
    })
}

#[get("/greeting?<country>")]
fn greeting(country: Option<String>) -> Json<Response> {
    let greeting = match country.as_deref() {
        Some("japan") => "こんにちは",
        Some("china") => "你好",
        Some("us") => "Hello",
        _ => "???",
    };
    Json(Response {
        message: greeting.to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![current_time, hello, greeting])
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    message: String,
}
