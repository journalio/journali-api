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

pub trait Find<By = Uuid>: Sized {
    fn find(id: By, connection: &PgConnection) -> QueryResult<Self>;
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

pub struct Crudder<T> {
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
// where
//     T: Send + Sync + Find + serde::Serialize + 'static,
//     By: Send + 'static
{
    pub async fn find<By>(id: By, pool: &DbPool) -> Result<HttpResponse, Error>
    where
        T: Send + Sync + Find<By> + serde::Serialize + 'static,
        By: Send + 'static,
    {
        Crudder::<T>::find_and_then(id, pool, |t| t).await
    }

    pub async fn find_and_then<F, O, By>(
        id: By,
        pool: &DbPool,
        f: F,
    ) -> Result<HttpResponse, Error>
    where
        T: Send + Sync + Find<By> + 'static,
        By: Send + 'static,
        O: Send + 'static,
        F: FnOnce(T) -> O,
        O: serde::Serialize,
    {
        exec_on_pool(pool, |conn| T::find(id, conn))
            .await
            .map(f)
            .into_response()
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
    T: Delete,
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
