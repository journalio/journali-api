use core::marker::PhantomData;

use diesel::{pg::PgConnection, QueryResult};
use uuid::Uuid;

use crate::utils::responsable::Responsable;
use crate::{database::exec_on_pool, DbPool};

use actix_web::{Error, HttpResponse};

pub trait Create: Sized {
    type Create: Send + Sync;

    fn create(
        create: &Self::Create,
        connection: &PgConnection,
    ) -> QueryResult<Self>;
}

pub trait Find: Sized {
    fn find(id: Uuid, connection: &PgConnection) -> QueryResult<Self>;
}

pub trait Update: Sized {
    type Update: Send + Sync;

    fn update(
        uuid: Uuid,
        update: &Self::Update,
        connection: &PgConnection,
    ) -> QueryResult<Self>;
}

pub trait Delete: Sized {
    fn delete(uuid: Uuid, connection: &PgConnection) -> QueryResult<()>;
}

pub struct Crudder<T: Send + Sync + 'static> {
    mark: PhantomData<T>,
}

impl<T> Crudder<T>
where
    T: Send + Sync + Create + serde::Serialize + 'static,
{
    pub async fn create(
        create: T::Create,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| T::create(&create, conn))
            .await
            .into_response()
    }
}

impl<T> Crudder<T>
where
    T: Send + Sync + Find + serde::Serialize + 'static,
{
    pub async fn find(id: Uuid, pool: &DbPool) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| T::find(id, conn)).await.into_response()
    }
}

impl<T> Crudder<T>
where
    T: Send + Sync + Update + serde::Serialize + 'static,
{
    pub async fn update(
        uuid: Uuid,
        update: T::Update,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| T::update(uuid, &update, conn))
            .await
            .into_response()
    }
}

impl<T> Crudder<T>
where
    T: Send + Sync + Delete + 'static,
{
    pub async fn delete(
        uuid: Uuid,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| T::delete(uuid, conn))
            .await
            .into_response()
    }
}
