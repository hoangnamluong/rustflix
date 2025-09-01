use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::casting::{Casting, CastingCreateDTO, CastingUpdateDTO};
use crate::schema::casting::{self, dsl};

impl Casting {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        casting::table.load::<Casting>(conn)
    }

    pub fn get_by_title(conn: &mut DatabaseConn, title_id: i32) -> QueryResult<Vec<Self>> {
        casting::table.filter(dsl::title_id.eq(title_id)).load::<Casting>(conn)
    }

    pub fn get_by_filmmaker(conn: &mut DatabaseConn, filmmaker_id: i32) -> QueryResult<Vec<Self>> {
        casting::table.filter(dsl::filmmaker_id.eq(filmmaker_id)).load::<Casting>(conn)
    }

    pub fn get_by_role(conn: &mut DatabaseConn, role_id: i32) -> QueryResult<Vec<Self>> {
        casting::table.filter(dsl::role_id.eq(role_id)).load::<Casting>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, casting: &CastingCreateDTO) -> QueryResult<usize> {
        insert_into(casting::dsl::casting).values(casting).execute(conn)
    }

    //Flag for review
    pub fn update(conn: &mut DatabaseConn, title_id: i32, filmmaker_id: i32, role_id: i32, casting: &CastingUpdateDTO) -> QueryResult<usize> {
        update(casting::table.filter(dsl::title_id.eq(title_id))
            .filter(dsl::filmmaker_id.eq(filmmaker_id))
            .filter(dsl::role_id.eq(role_id)))
            .set(casting)
            .execute(conn)
    }

    //Flag for review
    pub fn delete(conn: &mut DatabaseConn, title_id: i32, filmmaker_id: i32, role_id: i32) -> QueryResult<usize> {
        delete(casting::table.filter(dsl::title_id.eq(title_id))
            .filter(dsl::filmmaker_id.eq(filmmaker_id))
            .filter(dsl::role_id.eq(role_id)))
            .execute(conn)
    }
}
