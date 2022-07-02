use actix_web::{middleware::Logger, web::Data, App, HttpServer};

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(factory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    let port = 3030;
    println!("Serving the memory db at : {}", port);
    warp::serve(db.all()).run(([127, 0, 0, 1], port)).await;
}
