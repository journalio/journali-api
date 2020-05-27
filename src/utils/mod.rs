use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::database::exec_on_pool;
use crate::users::User;
use crate::utils::jwt::Jwt;
use crate::DbPool;

pub(crate) mod jwt;
pub(crate) mod responsable;

pub(crate) fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub async fn validator(
    req: ServiceRequest,
    _credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if req.path() == "/register" || req.path() == "/login" {
        return Ok(req);
    }

    let pool = req.app_data::<DbPool>().unwrap();

    exec_on_pool(&pool, move |conn| {
        let jwt = Jwt::decrypt(_credentials.token()).unwrap();
        User::find_by_id(&conn, jwt.sub())
    })
    .await
    .map(|user| {
        req.extensions_mut().insert(user);
        req
    })
    .map_err(ErrorUnauthorized)
}
