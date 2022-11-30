mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use api::todo_api::{
    create_todo,
    update_todo,
    get_todo,
    delete_todo,
};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_todo])
        .mount("/", routes![update_todo])
        .mount("/", routes![delete_todo])
        .mount("/", routes![get_todo])
}
