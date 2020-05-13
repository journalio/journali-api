use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::{pg::PgConnection, prelude::*, QueryResult};

use crate::schema::users;

#[derive(Queryable, Serialize, Insertable, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::login);
        cfg.service(routes::register);
    }
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

impl User {
    // TODO: Hashing
    fn create(conn: &PgConnection, newuser: &NewUser) -> QueryResult<Self> {
        diesel::insert_into(users::table).values(newuser).get_result(conn)
    }

    fn find_user(conn: &PgConnection, user: &LoginUser) -> QueryResult<Self> {
        users::table
            .filter(
                users::username
                    .eq(&user.username)
                    .and(users::password.eq(&user.password)),
            )
            .get_result(conn)
    }
}

impl User {
    fn into_jwt(self) -> String {
        use crate::utils::jwt::Jwt;
        use chrono::Duration;

        Jwt::new("journali.nl", Duration::days(30), self.id).tokenize()
    }
}

mod routes {
    use super::{LoginUser, NewUser, User};

    use actix_web::{
        post,
        web::{self},
        Error, HttpResponse,
    };

    use crate::{database::exec_on_pool, DbPool};

    #[post("/login")]
    pub(super) async fn login(
        pool: web::Data<DbPool>,
        user: web::Json<LoginUser>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| User::find_user(conn, &user))
            .await
            .map(User::into_jwt)
            .map(|jwt| HttpResponse::Ok().json(jwt))
            .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }

    #[post("/register")]
    pub(super) async fn register(
        pool: web::Data<DbPool>,
        new_user: web::Json<NewUser>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| User::create(conn, &new_user))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }
}
