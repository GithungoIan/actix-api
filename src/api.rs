use super::Pool;
use crate::handlers;
use crate::models::NewUser;
use crate::reponse::ResponseBody;
use actix_web::{web, HttpResponse, Result};

// GET /users
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match handlers::get_all_users(&pool) {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new("Ok", users))),
        Err(err) => Ok(err.response()),
    }
}

// GET /users/{id}
pub async fn find_by_id(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match handlers::get_user(id.into_inner(), &pool) {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", user))),
        Err(err) => Ok(err.response()),
    }
}

// POST /users
pub async fn add_user(new_user: web::Json<NewUser>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match handlers::add_new_user(new_user.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new("Ok", ""))),
        Err(err) => Ok(err.response()),
    }
}

// PUT /users/{id}
pub async fn update_user(
    id: web::Path<i32>,
    updated_user: web::Json<NewUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match handlers::update_user(id.into_inner(), updated_user.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("OK", ""))),
        Err(err) => Ok(err.response()),
    }
}

// DELETE /users/{id}
pub async fn delete_user(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match handlers::delet_user(id.into_inner(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("OK", ""))),
        Err(err) => Ok(err.response()),
    }
}
