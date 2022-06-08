use crate::{db, errors::MyError};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn get_fungible_tokens_list(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let tokens = db::get_all_fungible_tokens(&client).await?;

    Ok(HttpResponse::Ok().json(tokens))
}
