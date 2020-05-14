use diesel::{pg::PgConnection, prelude::*, QueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}

impl NewUser {
    fn hash_password(&self) -> Self {
        Self {
            password: crate::utils::hash_password(&self.password),
            username: self.username.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginUser {
    username: String,
    password: String,
}

impl User {
    // TODO: Hashing
    fn create(conn: &PgConnection, newuser: &NewUser) -> QueryResult<Self> {
        let newuser = newuser.hash_password();

        diesel::insert_into(users::table).values(newuser).get_result(conn)
    }

    fn find_user(
        conn: &PgConnection,
        loginuser: &LoginUser,
    ) -> QueryResult<Self> {
        users::table
            .filter(
                users::username.eq(&loginuser.username), //.filter(users::password.eq(&user.password)),
            )
            .load::<User>(conn)?
            .into_iter()
            .find(move |user| user.verify_password(loginuser))
            .ok_or_else(|| diesel::result::Error::NotFound)
    }
}

#[derive(Debug)]
struct InvalidPassword;

impl User {
    fn into_jwt(self) -> String {
        use crate::utils::jwt::Jwt;
        use chrono::Duration;

        Jwt::new("journali.nl", Duration::days(30), self.id).tokenize()
    }

    fn verify_password(&self, user: &LoginUser) -> bool {
        match bcrypt::verify(&user.password, &self.password) {
            Ok(true) => true,
            _ => false,
        }
    }
}

mod routes {
    use actix_web::{
        post,
        web::{self},
        Error, HttpResponse,
    };

    use crate::utils::responsable::Responsable;
    use crate::{database::exec_on_pool, DbPool};

    use super::{LoginUser, NewUser, User};

    #[post("/login")]
    pub(super) async fn login(
        pool: web::Data<DbPool>,
        user: web::Json<LoginUser>,
    ) -> Result<HttpResponse, Error> {
        let cloned_user = user.clone();

        exec_on_pool(pool, move |conn| User::find_user(conn, &cloned_user))
            .await
            .map(User::into_jwt)
            .into_response()
    }

    #[post("/register")]
    pub(super) async fn register(
        pool: web::Data<DbPool>,
        new_user: web::Json<NewUser>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| User::create(conn, &new_user))
            .await
            .into_response()
    }
}
