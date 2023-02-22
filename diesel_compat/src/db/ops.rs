use super::PoolConn;
use crate::{NewUserCompat, UserCompat};
use diesel::prelude::*;

impl<'u> NewUserCompat<'u> {
    pub fn insert<T>(&self, mut conn: PoolConn<T>) -> Result<UserCompat, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: UserCompat = diesel::insert_into(crate::user_t::table)
            .values(self)
            .get_result(&mut conn)?;
        return Ok(user.into());
    }
}
impl UserCompat {
    pub fn by_email<T>(
        email: String,
        mut conn: PoolConn<T>,
    ) -> Result<Option<UserCompat>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<UserCompat> = crate::user_t::table
            .filter(crate::user_t::email.eq(email))
            .first(&mut conn)
            .optional()?;
        return Ok(user);
    }
    pub fn by_id<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<UserCompat>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<UserCompat> = crate::user_t::table
            .filter(crate::user_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        return Ok(user);
    }
    pub fn delete<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<UserCompat>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<UserCompat> = crate::user_t::table
            .filter(crate::user_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        if let Some(user) = user {
            diesel::delete(crate::user_t::table.filter(crate::user_t::id.eq(id)))
                .execute(&mut conn)?;
            return Ok(Some(user));
        }
        return Ok(None);
    }
}