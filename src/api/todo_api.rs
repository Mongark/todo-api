use crate::{
    models::todo_model::Todo,
    repository::mongodb_repo::MongoRepo,
};
use mongodb::results::InsertOneResult;
use rocket::{
    http::Status,
    serde::json::Json,
    State,
};

#[post("/todo", data = "<new_todo>")]
pub fn create_todo(
    db: &State<MongoRepo>,
    new_todo: Json<Todo>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Todo {
        id: None,
        name: new_todo.name.to_owned(),
        checked: new_todo.checked.to_owned(),
    };
    let todo_detail = db.create_todo(data);
    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/todo/<path>")]
pub fn get_todo(
    db: &State<MongoRepo>,
    path: String,
) -> Result<Json<Todo>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let todo_detail = db.get_todo(&id);
    match todo_detail {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(Status::InternalServerError),
    }
}
