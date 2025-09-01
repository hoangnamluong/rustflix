use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::filmmaker::{Filmmaker, FilmmakerCreateDTO, FilmmakerUpdateDTO};
use crate::schema::filmmaker::{self, dsl};

impl Filmmaker {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        filmmaker::table.load::<Filmmaker>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        filmmaker::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn create(conn: &mut DatabaseConn, filmmaker: &FilmmakerCreateDTO) -> QueryResult<usize> {
        insert_into(filmmaker::dsl::filmmaker).values(filmmaker).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, filmmaker: &FilmmakerUpdateDTO) -> QueryResult<usize> {
        update(dsl::filmmaker.find(id)).set(filmmaker).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::filmmaker.find(id)).execute(conn)
    }
}
