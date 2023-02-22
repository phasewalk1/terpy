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

pub use self::NewUser as NewUserCompat;
pub use self::User as UserCompat;

impl<'u> From<prostgen::prostgen::NewUser> for NewUserCompat<'u> {
    fn from(user: prostgen::prostgen::NewUser) -> Self {
        let name: &mut str = Box::leak(user.name.into_boxed_str());
        let email: &mut str = Box::leak(user.email.into_boxed_str());
        let password_hash: &mut str = Box::leak(user.password_hash.into_boxed_str());
        return NewUserCompat {
            name,
            email,
            password_hash,
        };
    }
}

impl From<UserCompat> for prostgen::prostgen::User {
    fn from(user: UserCompat) -> Self {
        return prostgen::prostgen::User {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            password_hash: user.password_hash,
        };
    }
}
