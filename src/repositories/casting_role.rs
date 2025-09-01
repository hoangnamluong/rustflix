use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::casting_role::{CastingRole, CastingRoleCreateDTO, CastingRoleUpdateDTO};
use crate::schema::casting_role::{self, dsl};

impl CastingRole {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        casting_role::table.load::<CastingRole>(conn)
    }

    pub fn get_by_id(conn: &mut DatabaseConn, id: i32) -> QueryResult<Self> {
        casting_role::table.filter(dsl::id.eq(id)).first(conn)
    }

    pub fn create(conn: &mut DatabaseConn, role: &CastingRoleCreateDTO) -> QueryResult<usize> {
        insert_into(casting_role::dsl::casting_role).values(role).execute(conn)
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, role: &CastingRoleUpdateDTO) -> QueryResult<usize> {
        update(dsl::casting_role.find(id)).set(role).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        delete(dsl::casting_role.find(id)).execute(conn)
    }
}
