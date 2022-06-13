mod config;
mod models;
mod errors;
mod db;
mod handlers;
mod tasks;
mod rpc;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::get_fungible_tokens_list;
use crate::tasks::TaskManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.fp_pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(web::resource("/token-list").route(web::get().to(get_fungible_tokens_list)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    let cfg = Arc::new(config);

    TaskManager::new(cfg.clone())
        .await
        .run()
        .await;

    server.await
}
