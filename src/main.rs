#[macro_use]
extern crate diesel;

use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::Error;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger;
use log::info;
use std::pin::Pin;

mod api;
mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod reponse;
mod routes;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", " actix_web=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL mut be set");

    // CREATE A DB CONNECTION POOL
    let manager = ConnectionManager::<db::Connection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed tp create pool");

    info!("App is running on port 5000");

    // actix server
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(routes::config_services)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
