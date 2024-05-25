#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::response::status::{self, Ok as OK};
use sea_orm::*;

mod catchers;
use catchers::{not_found, unprocessable};

mod fairings;
use fairings::CORS;

mod routes;
use routes::prelude::*;

mod entities;
// This is needed for other files that are dependant on it
#[allow(unused_imports)]
use entities::{prelude::*, *};

#[get("/")]
fn index() -> status::Ok<&'static str> {
    OK("Hey!")
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let url = std::env::var("DB_CONNECTION_STRING").expect("Database connection string must be set!");
    let connection =
        match Database::connect(url).await {
            Ok(connection) => connection,
            Err(e) => {
                panic!("Error connecting to database: {}", e);
            }
        };

    rocket::build()
        .attach(CORS)
        .manage(connection)
        .mount("/", routes![index])
        .mount("/questions", routes![get_all_questions, create_question])
        .mount("/answers", routes![get_all_answers, create_answer])
        .mount("/categories", routes![get_all_categories, create_category])
        .register("/", catchers![not_found])
        .register("/quiz", catchers![unprocessable])
}
