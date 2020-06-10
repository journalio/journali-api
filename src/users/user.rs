use diesel::{pg::PgConnection, prelude::*, QueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

use crate::items::crud::{Create, Find};

#[derive(
    Identifiable, Queryable, Deserialize, Serialize, Insertable, Debug, Clone,
)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::login);
        cfg.service(routes::register);
        cfg.service(routes::update_user);
    }

    pub fn route_me(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::me);
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

#[derive(AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    username: Option<String>,
    password: Option<String>,
}

impl Create for User {
    type Create = NewUser;

    fn create(
        new_user: &Self::Create,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let new_user = new_user.hash_password();

        diesel::insert_into(users::table).values(new_user).get_result(conn)
    }
}

impl<'a> Find<&'a LoginUser> for User {
    fn find(
        loginuser: &'a LoginUser,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        users::table
            .filter(users::username.eq(&loginuser.username))
            .load::<User>(conn)?
            .into_iter()
            .find(move |user| user.verify_password(loginuser))
            .ok_or_else(|| diesel::result::Error::NotFound)
    }
}

impl UpdateUser {
    fn hash_password(self) -> Self {
        let password =
            self.password.as_ref().map(|s| crate::utils::hash_password(s));

        Self { password, username: self.username }
    }
}

impl User {
    fn update(
        id: Uuid,
        conn: &PgConnection,
        update_user: UpdateUser,
    ) -> QueryResult<Self> {
        let update_user = update_user.hash_password();

        diesel::update(users::table.filter(users::id.eq(id)))
            .set(update_user)
            .get_result(conn)
    }

    pub fn find_by_id(conn: &PgConnection, id: Uuid) -> QueryResult<Self> {
        users::table.filter(users::id.eq(id)).first::<User>(conn)
    }
}

#[derive(Debug)]
struct InvalidPassword;

impl User {
    fn into_jwt(self) -> crate::utils::jwt::Token {
        use crate::utils::jwt::Jwt;
        use chrono::Duration;

        Jwt::new("journali.nl".to_string(), Duration::days(30), self.id)
            .tokenize()
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
        get, patch, post,
        web::{self},
        Error, HttpRequest, HttpResponse,
    };

    use crate::utils::responsable::Responsable;
    use crate::{database::exec_on_pool, DbPool};

    use super::{LoginUser, NewUser, UpdateUser, User};
    use uuid::Uuid;

    use crate::items::crud::{Crudder, Find};

    #[post("/login")]
    pub(super) async fn login(
        pool: web::Data<DbPool>,
        user: web::Json<LoginUser>,
    ) -> Result<HttpResponse, Error> {
        let cloned_user = user.clone();

        exec_on_pool(&pool, move |conn| User::find(&cloned_user, conn))
            .await
            .map(User::into_jwt)
            .into_response()
    }

    #[post("/register")]
    pub(super) async fn register(
        pool: web::Data<DbPool>,
        new_user: web::Json<NewUser>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<User>::create(new_user.into_inner(), &pool).await
    }

    #[get("/user/me")]
    pub(super) async fn me(
        request: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        let user: User = request.extensions().get().cloned().unwrap();
        Ok(HttpResponse::Ok().json(user))
    }

    #[patch("/users/{id}")]
    pub async fn update_user(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        update_user: web::Json<UpdateUser>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(&pool, move |conn| {
            User::update(id.into_inner(), conn, update_user.into_inner())
        })
        .await
        .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::routes;
    use super::{LoginUser, NewUser, User};
    use actix_web::{
        http::StatusCode,
        test,
        test::{read_response_json, call_service, TestRequest},
    };

    fn build_request<T: serde::Serialize>(uri: &str, json: &T) -> TestRequest {
        test::TestRequest::post().uri(uri).set_json(json)
    }
    
    fn register_request(username: &str, password: &str) -> TestRequest {
        build_request("/register", &NewUser { username: username.into(), password: password.into() })
    }

    #[actix_rt::test]
    async fn test_register() -> Result<(), Box<dyn std::error::Error>> {
        test! {
            setup {
                |cfg| { cfg.service(routes::register); }
            }

            test = |app| {
                const USER_NAME: &str = "sailor jack";
                const PASSWORD: &str = "black pearl";

                let request = register_request(USER_NAME, PASSWORD).to_request();

                let user: User = read_response_json(&mut app, request).await;

                assert_eq!(user.username, USER_NAME);
                let passwd_verify = bcrypt::verify(PASSWORD, &user.password)?;

                assert!(passwd_verify);

                Ok(())
            }
        }
    }

    #[actix_rt::test]
    async fn test_login() -> Result<(), Box<dyn std::error::Error>> {
        test! {
            setup {
                |cfg| {
                    cfg.service(routes::register);
                    cfg.service(routes::login);
                }
            }

            test = |app| {
                const USER_NAME: &str = "sailor2";
                const PASSWORD: &str = "black pearl";

                // Need to register before login
                {
                    let request = register_request(USER_NAME, PASSWORD).to_request();

                    let resp = call_service(&mut app, request).await;
                    assert_eq!(resp.status(), StatusCode::OK);
                }

                // the actual login
                {
                    let request = build_request(
                        "/login",
                        &LoginUser {
                            username: USER_NAME.into(),
                            password: PASSWORD.into(),
                        },
                    )
                    .to_request();

                    let resp = call_service(&mut app, request).await;
                    assert_eq!(resp.status(), StatusCode::OK);
                }
                Ok(())
            }
        }
    }
}
