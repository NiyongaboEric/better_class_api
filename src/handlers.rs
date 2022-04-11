use actix_web::Responder;

pub async fn add_user() -> impl Responder {
    format!("Hello from add user")
}
