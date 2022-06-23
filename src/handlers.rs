use super::models::{NewUser, User};
use super::Pool;
use crate::errors::ServiceError;
use actix_web::{http::StatusCode, web};
use std::vec::Vec;



// handler for GET /users
pub fn get_all_users(pool: &web::Data<Pool>) -> Result<Vec<User>, ServiceError> {
    match User::find_all(&pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to fetch users".to_string(),
        )),
    }
}

// handler for POST /users
pub fn add_new_user(new_user: NewUser, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match User::insert(new_user, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to Insert new user".to_string(),
        )),
    }
}

// handler for GET /users/{id}
pub fn get_user(find_id: i32, pool: &web::Data<Pool>) -> Result<User, ServiceError> {
    match User::find_by_id(find_id, &pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("user with  {} id NOT FOUND", find_id),
        )),
    }
}

// handler for UPDATE /users/{id}
pub fn update_user(
    id: i32,
    updated_user: NewUser,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match User::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match User::update(id, updated_user, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update user".to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("User with {}id not found", id),
        )),
    }
}

// handler for DELETE /users/{id}
pub fn delet_user(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match User::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match User::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete user".to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("User with id {} Not found", id),
        )),
    }
}
