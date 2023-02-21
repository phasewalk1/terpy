use crate::diesel_schema::users as users_t;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users_t)]
pub struct NewUser<'u> {
    pub name: &'u str,
    pub email: &'u str,
    pub password_hash: &'u str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = users_t)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
