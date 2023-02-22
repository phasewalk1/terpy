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

use crate::models::grower::{NewTestResults, TestResults};
impl NewTestResults {
    pub fn insert<T>(&self, mut conn: PoolConn<T>) -> Result<TestResults, diesel::result::Error>
    where
        T: diesel::r2d2::ManageConnection<Connection = diesel::pg::PgConnection>,
    {
        let test_results: TestResults = diesel::insert_into(crate::test_results_t::table)
            .values(self)
            .get_result(&mut conn)?;
        return Ok(test_results.into());
    }
}
