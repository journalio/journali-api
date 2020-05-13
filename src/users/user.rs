use serde::{Deserialize, Serialize};
use uuid::Uuid;

use diesel::{pg::PgConnection, prelude::*, QueryResult};

use jwt::{
    Header,
    Registered,
    Token,
};

use crypto::sha2::Sha256;

use crate::schema::users;


#[derive(Queryable, Serialize, Insertable, Debug)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub password_hash: String,
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
    full_name: String,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    full_name: String,
    password_hash: String,
}

impl User {
    // TODO: Hashing
    fn create(conn: &PgConnection, newuser: &NewUser) -> QueryResult<Self> {
        diesel::insert_into(users::table).values(newuser).get_result(conn)
    }

    fn find_user(conn: &PgConnection, user: &LoginUser) -> QueryResult<Self> {
        users::table
            .filter(
                users::full_name
                    .eq(&user.full_name)
                    .and(users::password_hash.eq(&user.password_hash)),
            )
            .get_result(conn)
    }
}

impl User {
    fn to_jwt(self) -> Token {
        let claims = Registered {
            sub: Some(self.password_hash),
            ..Default::default()
        };

        let token = Token::new(header, claims);
        let jwt = token.signed(AUTH_SECRET.as_bytes(), Sha256::new()).unwrap();
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
        let user = exec_on_pool(pool, move |conn| User::find_user(conn, &user))
            .await
            .map_err(|_| HttpResponse::InternalServerError().finish())?;
        
        // MOET JWT HIERZO?!!!!
        let token = user.to_jwt();

        Ok(HttpResponse::Ok().json(token))
    }

    #[post("/register")]
    pub(super) async fn register(
        pool: web::Data<DbPool>,
        new_user: web::Json<NewUser>,
    ) -> Result<HttpResponse, Error> {
        let user =
            exec_on_pool(pool, move |conn| User::create(conn, &new_user))
                .await
                .map_err(|_| HttpResponse::InternalServerError().finish())?;
        Ok(HttpResponse::Ok().json(user))
    }
}
