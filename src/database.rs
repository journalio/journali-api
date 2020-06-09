use core::{
    fmt::{Debug, Display},
    future::Future,
};

use actix_web::{web, ResponseError};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub(crate) fn exec_on_pool<T, E, F>(
    pool: &DbPool,
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

#[actix_rt::test]
async fn test_database_connection_works() {
    use diesel::pg;

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<pg::PgConnection>::new(conn_spec);

    let pool: DbPool =
        r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    assert_eq!(pool.get().is_ok(), true);
}
