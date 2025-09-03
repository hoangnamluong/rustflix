use diesel::{pg::Pg, sql_query, QueryableByName, RunQueryDsl};

use crate::config::db_config::DatabaseConn;

#[derive(QueryableByName, Debug)]
struct ScalarText {
    #[diesel(sql_type = diesel::sql_types::Text)]
    value: String
}

#[derive(QueryableByName, Debug)]
struct ScalarInt {
    #[diesel(sql_type = diesel::sql_types::Integer)]
    value: i32
}

pub fn select<T>(conn: &mut DatabaseConn, query: &str) -> Result<Vec<T>, String>
where
    T: QueryableByName<Pg> + 'static,
{
    let sql_query = sql_query(query);
    
    sql_query.load::<T>(conn).map_err(|err| format!("db_helper::select::error: {}", err.to_string()))
}

pub fn scalar_text(conn: &mut DatabaseConn, query: &str) -> Result<String, String>
{
    let sql_query = sql_query(query);
    let value: Vec<ScalarText> = sql_query.load(conn).map_err(|err| format!("db_helper::select::error: {}", err.to_string()))?;

    match value.len() {
        0 => return Err(format!("db_helper::scalar::error: Empty result")),
        1 => return Ok(value.into_iter().next().unwrap().value),
        _ => return Err(format!("db_helper::scalar::error: Multiple values"))
    }
}

pub fn scalar_int(conn: &mut DatabaseConn, query: &str) -> Result<i32, String>
{
    let sql_query = sql_query(query);
    let value: Vec<ScalarInt> = sql_query.load(conn).map_err(|err| format!("db_helper::select::error: {}", err.to_string()))?;

    match value.len() {
        0 => return Err(format!("db_helper::scalar::error: Empty result")),
        1 => return Ok(value.into_iter().next().unwrap().value),
        _ => return Err(format!("db_helper::scalar::error: Multiple values"))
    }
}