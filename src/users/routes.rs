// #[post("/users")]
// async fn create(user: web::JSON) -> Result<HttpResponse, CustomError> {
//     let employee = Users::create(user.into_inner())?
//     Ok(HttpResponse::Ok().json(employee));
// }