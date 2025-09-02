use diesel::dsl::{delete, insert_into, update};
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, ExpressionMethods, QueryResult, RunQueryDsl};
use crate::config::db_config::DatabaseConn;
use crate::models::profile::{MaturityRating, Profile, ProfileCreateDTO};
use crate::models::users::{UserStatus, Users, UsersCreateDTO, UsersUpdateDTO};
use crate::schema::profile;
use crate::schema::users::{self, dsl};

impl Users {
    pub fn get_all(conn: &mut DatabaseConn) -> QueryResult<Vec<Self>> {
        users::table.load::<Users>(conn)
    }

    pub fn get_by_id(conn:& mut DatabaseConn, id: i32) -> QueryResult<Self> {
        users::table.filter(dsl::id.eq(id)).first(conn)
    } 

    pub fn create(conn: &mut DatabaseConn, user: &UsersCreateDTO) -> QueryResult<usize> {
        conn.transaction::<usize, diesel::result::Error, _>(|conn| {
            let user: Users = insert_into(users::dsl::users).values(user).get_result(conn)?;
            let profile: Profile = insert_into(profile::dsl::profile)
                .values(ProfileCreateDTO {
                    user_id: user.id,
                    name: "Username".to_string(),
                    maturity_rating_max: MaturityRating::PG13,
                    language: 1,
                    avatar_url: None
                })
                .get_result(conn)?;

            Ok(1)
        })
    }

    pub fn update(conn: &mut DatabaseConn, id: i32, user: &UsersUpdateDTO) -> QueryResult<usize> {
        update(dsl::users.find(id)).set(user).execute(conn)
    }

    pub fn delete(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        update(dsl::users.find(id)).set(dsl::status.eq(UserStatus::INACTIVE)).execute(conn)
    }

    pub fn ban(conn: &mut DatabaseConn, id: i32) -> QueryResult<usize> {
        update(dsl::users.find(id)).set(dsl::status.eq(UserStatus::BANNED)).execute(conn)
    }
}