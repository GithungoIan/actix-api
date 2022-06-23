use crate::schema::users::id;
use crate::schema::users::{self, dsl::*};
use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
pub type Connection = PgConnection;

use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<User>> {
        users.order(id.asc()).load::<User>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<User> {
        users.find(i).get_result::<User>(conn)
    }

    pub fn insert(new_user: NewUser, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(users).values(&new_user).execute(conn)
    }

    pub fn update(i: i32, update_user: NewUser, conn: &Connection) -> QueryResult<usize> {
        diesel::update(users.find(i))
            .set(&update_user)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(users.find(i)).execute(conn)
    }
}
