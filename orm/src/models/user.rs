use crate::auto::users as user_t;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub use self::NewUser as InsertableUser;
pub use self::User as SearchableUser;

/// An ORM insertable user
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = user_t)]
pub struct NewUser<'u> {
    pub name: &'u str,
    pub email: &'u str,
    pub is_grower: bool,
    pub password_hash: &'u str,
}

/// An ORM searchable user
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = user_t)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_grower: bool,
    pub password_hash: String,
}

impl<'u> From<prostgen::user::NewUser> for InsertableUser<'u> {
    fn from(user: prostgen::user::NewUser) -> Self {
        let name: &mut str = Box::leak(user.name.into_boxed_str());
        let email: &mut str = Box::leak(user.email.into_boxed_str());
        let password_hash: &mut str = Box::leak(user.password_hash.into_boxed_str());
        let is_grower: bool = user.is_grower;
        return InsertableUser {
            name,
            email,
            is_grower,
            password_hash,
        };
    }
}

impl From<SearchableUser> for prostgen::user::User {
    fn from(user: SearchableUser) -> Self {
        return prostgen::user::User {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            is_grower: user.is_grower,
            password_hash: user.password_hash,
        };
    }
}
