use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::title::{Title, TitleCreateDTO, TitleUpdateDTO};
use crate::schema::title::{self, dsl};

impl Title {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        title::table.load::<Title>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        title::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn get_by_language(conn: &mut DatabaseConn, language_id: i32) -> QueryResult<Vec<Self>> {
        title::table.filter(dsl::orginal_language_id.eq(language_id)).load::<Title>(conn)
    }

    pub fn create(conn: &mut DatabaseConn, title: &TitleCreateDTO) -> QueryResult<usize> {
        insert_into(title::dsl::title).values(title).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, title: &TitleUpdateDTO) -> QueryResult<usize> {
        update(dsl::title.find(id)).set(title).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::title.find(id)).execute(conn)
    }
}
