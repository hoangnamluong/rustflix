use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::genre::{Genre, GenreCreateDTO, GenreUpdateDTO};
use crate::schema::genre::{self, dsl};

impl Genre {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        genre::table.load::<Genre>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        genre::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn create(conn: &mut DatabaseConn, genre: &GenreCreateDTO) -> QueryResult<usize> {
        insert_into(genre::dsl::genre).values(genre).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, genre: &GenreUpdateDTO) -> QueryResult<usize> {
        update(dsl::genre.find(id)).set(genre).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::genre.find(id)).execute(conn)
    }
}
