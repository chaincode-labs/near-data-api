mod config;
mod models;
mod errors;
mod db;
mod handlers;
mod tasks;
mod rpc;

use actix_web::{ web, App, HttpServer};
use actix_web::rt::spawn;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Hello World!" }))

    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    crate::tasks::run(&config).await;

    server.await
}
