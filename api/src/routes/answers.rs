use rocket::State;
use rocket::response::status::{self, BadRequest};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use sea_orm::*;

use crate::entities::{prelude::*, *};

#[derive(Debug, Deserialize)]
pub struct AnswerInputData {
    text: String,
    is_correct: Option<bool>
}

#[get("/")]
pub async fn get_all_answers(connection: &State<DatabaseConnection>) -> Result<Json<Vec<answer::Model>>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let result = Answer::find().all(connection).await;

    match result {
        Ok(_) => Ok(result.map(Json).unwrap()),
        Err(e) => Err(BadRequest(format!("There was an error getting all answers: {}", e)))
    }
}

#[post("/", data = "<input>")]
pub async fn create_answer(connection: &State<DatabaseConnection>, input: Json<AnswerInputData>) -> Result<status::Ok<&'static str>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let data = input.into_inner();

    let record = answer::ActiveModel {
        text: Set(data.text),
        is_correct: Set(data.is_correct.unwrap_or(false)),
        ..Default::default()
    };

    let result = Answer::insert(record.clone()).exec(connection).await;

    match result {
        Ok(_) => Ok(status::Ok("Successfully created answer")),
        Err(e) => Err(BadRequest(format!("There was an error creating answer: {}", e)))
    }
}
