mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use api::todo_api::create_todo;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_todo])
}
