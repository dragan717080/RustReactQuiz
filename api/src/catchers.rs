use rocket::Request;

#[catch(422)]
pub fn unprocessable() -> &'static str {
    "Params supplied were incorrect"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Route {} was not found", req.uri())
}
