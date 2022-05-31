use crate::{db, errors::MyError};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn get_top_list(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    Ok(HttpResponse::Ok().json(
        db::get_top_list(&client).await?
    ))
}