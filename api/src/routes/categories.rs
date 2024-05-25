use rocket::State;
use rocket::response::status::{self, BadRequest};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use sea_orm::*;

use crate::entities::{prelude::*, *};

#[derive(Debug, Deserialize)]
pub struct CategoryInputData {
    name: String,
}

#[get("/")]
pub async fn get_all_categories(connection: &State<DatabaseConnection>) -> Result<Json<Vec<category::Model>>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let result = Category::find().all(connection).await;
    
    match result {
        Ok(_) => Ok(result.map(Json).unwrap()),
        Err(e) => Err(BadRequest(format!("There was an error getting all categories: {}", e)))
    }
}

#[post("/", data = "<input>")]
pub async fn create_category(connection: &State<DatabaseConnection>, input: Json<CategoryInputData>) -> Result<status::Ok<&'static str>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let data = input.into_inner();

    let record = category::ActiveModel {
        name: Set(data.name),
        ..Default::default()
    };

    let result = Category::insert(record.clone()).exec(connection).await;
    
    match result {
        Ok(_) => Ok(status::Ok("Category successfully created")),
        Err(e) => Err(BadRequest(format!("There was an error creating category: {}", e)))
    }
}
