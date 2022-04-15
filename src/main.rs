#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let db_connection = rusqlite::Connection::open("data.sqlite");

    rocket::build().mount("/", routes![index]).launch();
}

