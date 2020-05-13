use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web::{web, ResponseError};

use core::{
    fmt::{Debug, Display},
    future::Future,
};

pub(crate) fn exec_on_pool<T, E, F>(
    pool: web::Data<DbPool>,
    f: F,
) -> impl Future<Output = Result<T, impl Debug + Display + ResponseError>>
where
    T: Send + 'static,
    E: Send + 'static + Debug,
    F: Send + 'static + FnOnce(&PgConnection) -> Result<T, E>,
{
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || f(&conn))
}
