mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;

use api::todo_api::{
    get_all_todos,
    create_todo,
    update_todo,
    delete_todo,
    get_todo,
};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![get_all_todos])
        .mount("/", routes![create_todo])
        .mount("/", routes![update_todo])
        .mount("/", routes![delete_todo])
        .mount("/", routes![get_todo])
}
