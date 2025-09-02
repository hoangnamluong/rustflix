use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::rating::{Rating, RatingCreateDTO, RatingUpdateDTO};
use crate::schema::rating::{self, dsl};

impl Rating {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        rating::table.load::<Rating>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        rating::table.filter(dsl::id.eq(id)).first(conn)
    }

    // pub fn get_by_user(conn: &mut DatabaseConn, user_id: i32) -> QueryResult<Vec<Self>> {
    //     rating::table.filter(dsl::user_id.eq(user_id)).load::<Rating>(conn)
    // }

    pub fn get_by_title(conn: &mut DatabaseConn, title_id: i32) -> QueryResult<Vec<Self>> {
        rating::table.filter(dsl::title_id.eq(title_id)).load(conn)
    }

    pub fn get_by_user_and_title(conn: &mut DatabaseConn, user_id: i32, title_id: i32) -> QueryResult<Vec<Self>> {
        rating::table
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::title_id.eq(title_id))
            .load(conn)
    }

    pub fn create(conn: &mut DatabaseConn, rating: &RatingCreateDTO) -> QueryResult<usize> {
        insert_into(rating::dsl::rating).values(rating).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, rating: &RatingUpdateDTO) -> QueryResult<usize> {
        update(dsl::rating.find(id)).set(rating).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::rating.find(id)).execute(conn)
    }
}
