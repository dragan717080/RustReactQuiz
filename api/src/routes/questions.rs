use rocket::State;
use rocket::response::status::{self, BadRequest};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use sea_orm::*;

use crate::entities::{prelude::*, *};

#[derive(Debug, Deserialize)]
pub struct AnswerInputData {
    text: String,
    is_correct: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct QuestionInputData {
    category_name: String,
    text: String,
    answers: [AnswerInputData; 4]
}

#[derive(Serialize)]
pub struct QuestionWithAnswers {
    pub question: question::Model,
    pub answers: Vec<answer::Model>,
}

#[get("/")]
pub async fn get_all_questions(connection: &State<DatabaseConnection>) -> Result<Json<Vec<QuestionWithAnswers>>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let result = Question::find().find_with_related(Answer).all(connection).await;

    match result {
        Ok(questions_with_answers) => {
            let combined_results: Vec<QuestionWithAnswers> = questions_with_answers.into_iter()
                .map(|(question, answers)| QuestionWithAnswers {
                    question,
                    answers,
                })
                .collect();
            Ok(Json(combined_results))
        },
        Err(e) => Err(BadRequest(format!("There was an error getting all questions: {}", e)))
    }
}

#[post("/", data = "<input>")]
pub async fn create_question(connection: &State<DatabaseConnection>, input: Json<QuestionInputData>) -> Result<status::Ok<&'static str>, BadRequest<String>> {
    let connection = connection as &DatabaseConnection;

    let data = input.into_inner();

    // Save question text, then get its id to save answers with FK
    let record = question::ActiveModel {
        text: Set(data.text),
        category_name: Set(data.category_name),
        ..Default::default()
    };

    let result = Question::insert(record.clone()).exec(connection).await;
    
    match result {
        Ok(result) => {
            let created_answers = create_answers(connection, result.last_insert_id, data.answers).await;

            if let Err(e) = created_answers {
                return Err(BadRequest(format!("Malformed answers: {}", e)))
            }

            Ok(status::Ok("Question successfully created"))
        },
        Err(e) => Err(BadRequest(format!("There was an error creating question: {}", e)))
    }
}

/**
 * Given the 'question_id' and the answers input data, create answers.
 */
async fn create_answers(connection: &DatabaseConnection, question_id: i32, answers: [AnswerInputData; 4]) -> Result<(), DbErr> {
    for answer in answers {
        let record = answer::ActiveModel {
            question_id: Set(question_id),
            text: Set(answer.text),
            is_correct: Set(answer.is_correct.unwrap_or(false)),
            ..Default::default()
        };
        
        let result = Answer::insert(record.clone()).exec(connection).await;

        if let Err(e) = result {
            return Err(e);
        }
    }

    Ok(())
}
