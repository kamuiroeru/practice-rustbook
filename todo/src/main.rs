use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

impl ResponseError for MyError {}

// MyError は ResponseError を実装しているので、返り値に使える
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";
    println!("request comming / !!!");
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
