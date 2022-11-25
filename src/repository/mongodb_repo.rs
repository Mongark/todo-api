use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{ extjson::de::Error, oid::ObjectId, doc },
    results::{ InsertOneResult },
    sync::{ Client, Collection },
};
use crate::models::todo_model::Todo;

pub struct MongoRepo {
    col: Collection<Todo>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v)   => v.to_string(),
            Err(_)  => format!("Error loading env variable"),
        };
        let client                  = Client::with_uri_str(uri).unwrap();
        let db                      = client.database("mydb");
        let col: Collection<Todo>   = db.collection("Todo");
        MongoRepo { col }
    }

    pub fn create_todo(&self, new_todo: Todo) -> Result<InsertOneResult, Error> {
        let new_doc = Todo {
            id: None,
            name: new_todo.name,
            checked: new_todo.checked,
        };
        let todo = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(todo)
    }

    pub fn get_todo(&self, id: &String) -> Result<Todo, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let todo_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting todo's detail");
        Ok(todo_detail.unwrap())
    }
}
