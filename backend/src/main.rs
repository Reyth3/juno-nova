use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cosmrs::rpc::{Client, HttpClient};

#[get("/")]
async fn hello() -> impl Responder {
    let client = HttpClient::new("https://rpc.uni.juno.deuslabs.fi").unwrap();
    let info = client.consensus_state().await;
    match info {
        Ok(x) => HttpResponse::Ok().body(format!("{:?}", x)),
        Err(x) => HttpResponse::Ok().body(format!("{:?}", x)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}
