use crate::prelude::*;
use super::pooling::pool::PoolConn;
use crate::auto::users as user_t;
use diesel::prelude::*;

impl<'u> InsertableUser<'u> {
    pub fn insert<T>(&self, mut conn: PoolConn<T>) -> Result<SearchableUser, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: SearchableUser = diesel::insert_into(user_t::table)
            .values(self)
            .get_result(&mut conn)?;
        return Ok(user.into());
    }
}
impl SearchableUser {
    pub fn by_email<T>(
        email: String,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableUser>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<SearchableUser> = user_t::table
            .filter(user_t::email.eq(email))
            .first(&mut conn)
            .optional()?;
        return Ok(user);
    }

    pub fn by_name<T>(
        name: String,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableUser>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<SearchableUser> = user_t::table
            .filter(user_t::name.eq(name))
            .first(&mut conn)
            .optional()?;
        return Ok(user);
    }

    pub fn by_id<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableUser>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<SearchableUser> = user_t::table
            .filter(user_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        return Ok(user);
    }
    pub fn delete<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableUser>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let user: Option<SearchableUser> = user_t::table
            .filter(user_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        if let Some(user) = user {
            diesel::delete(user_t::table.filter(user_t::id.eq(id)))
                .execute(&mut conn)?;
            return Ok(Some(user));
        }
        return Ok(None);
    }
}

impl InsertableCannibanoidScreen {
    pub fn insert<T>(
        &self,
        mut conn: PoolConn<T>,
    ) -> Result<SearchableCannibanoidScreen, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: SearchableCannibanoidScreen =
            diesel::insert_into(crate::auto::cannibanoid_screen_t::table)
                .values(self)
                .get_result(&mut conn)?;
        return Ok(screen.into());
    }
}

impl SearchableCannibanoidScreen {
    pub fn by_id<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableCannibanoidScreen>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: Option<SearchableCannibanoidScreen> = crate::auto::cannibanoid_screen_t::table
            .filter(crate::auto::cannibanoid_screen_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        return Ok(screen);
    }
    pub fn delete<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableCannibanoidScreen>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: Option<SearchableCannibanoidScreen> = crate::auto::cannibanoid_screen_t::table
            .filter(crate::auto::cannibanoid_screen_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        if let Some(screen) = screen {
            diesel::delete(
                crate::auto::cannibanoid_screen_t::table
                    .filter(crate::auto::cannibanoid_screen_t::id.eq(id)),
            )
            .execute(&mut conn)?;
            return Ok(Some(screen));
        }
        return Ok(None);
    }
}

impl InsertableTerpenoidScreen {
    pub fn insert<T>(
        &self,
        mut conn: PoolConn<T>,
    ) -> Result<SearchableTerpenoidScreen, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: SearchableTerpenoidScreen =
            diesel::insert_into(crate::auto::terpenoid_screen_t::table)
                .values(self)
                .get_result(&mut conn)?;
        return Ok(screen.into());
    }
}

impl SearchableTerpenoidScreen {
    pub fn by_id<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableTerpenoidScreen>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: Option<SearchableTerpenoidScreen> = crate::auto::terpenoid_screen_t::table
            .filter(crate::auto::terpenoid_screen_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        return Ok(screen);
    }
    pub fn delete<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableTerpenoidScreen>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let screen: Option<SearchableTerpenoidScreen> = crate::auto::terpenoid_screen_t::table
            .filter(crate::auto::terpenoid_screen_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        if let Some(screen) = screen {
            diesel::delete(
                crate::auto::terpenoid_screen_t::table
                    .filter(crate::auto::terpenoid_screen_t::id.eq(id)),
            )
            .execute(&mut conn)?;
            return Ok(Some(screen));
        }
        return Ok(None);
    }
}

impl InsertableTestResults {
    pub fn insert<T>(
        &self,
        mut conn: PoolConn<T>,
    ) -> Result<SearchableTestResults, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let results: SearchableTestResults =
            diesel::insert_into(crate::auto::test_results_t::table)
                .values(self)
                .get_result(&mut conn)?;
        return Ok(results.into());
    }
}

impl SearchableTestResults {
    pub fn by_id<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableTestResults>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let results: Option<SearchableTestResults> = crate::auto::test_results_t::table
            .filter(crate::auto::test_results_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        return Ok(results);
    }
    pub fn delete<T>(
        id: i32,
        mut conn: PoolConn<T>,
    ) -> Result<Option<SearchableTestResults>, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let results: Option<SearchableTestResults> = crate::auto::test_results_t::table
            .filter(crate::auto::test_results_t::id.eq(id))
            .first(&mut conn)
            .optional()?;
        if let Some(results) = results {
            diesel::delete(
                crate::auto::test_results_t::table.filter(crate::auto::test_results_t::id.eq(id)),
            )
            .execute(&mut conn)?;
            return Ok(Some(results));
        }
        return Ok(None);
    }
}
