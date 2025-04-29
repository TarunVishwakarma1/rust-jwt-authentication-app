use actix_web::{web::{get, post}, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/public-view", get().to( || async {"Public View"}))
            .route("/get-token", post().to(|| async {"Get Token"}))
            .route("/secret-view", get().to(|| async {"Secret View"}))
    })
        .workers(4)
        .bind("127.0.0.1:2424")
        .expect("Address should be free and valid")
        .run()
        .await
}