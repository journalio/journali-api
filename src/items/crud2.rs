use crate::items::item::Item;

pub(crate) mod raw_crud {
    use diesel::pg::PgConnection;
    use diesel::result::QueryResult;
    use uuid::Uuid;

    pub trait Create: Sized {
        fn create(self, conn: &PgConnection) -> QueryResult<Self>;
    }

    pub trait Find: Sized {
        fn find(key: Uuid, conn: &PgConnection) -> QueryResult<Self>;
    }

    pub trait Update<U>: Sized {
        fn update(
            id: Uuid,
            update: U,
            conn: &PgConnection,
        ) -> QueryResult<Self>;
    }

    pub trait Delete: Sized {
        fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()>;
    }
}

pub trait ModelFromPartial<P> {
    fn from_partial(partial: P, item: &Item) -> Self;
}

pub trait IntoModel<M> {
    fn into_model(self, item: &Item) -> M;
}

impl<P, M> IntoModel<M> for P
where
    M: ModelFromPartial<P>,
{
    fn into_model(self, item: &Item) -> M {
        M::from_partial(self, item)
    }
}

pub(self) mod intermidiate {
    use super::raw_crud;
    use super::IntoModel;

    use crate::items::{ItemLike, TypeMarker};
    use crate::users::user::User;

    use crate::items::item::Item;

    use diesel::{pg::PgConnection, QueryResult};
    use uuid::Uuid;

    pub fn create<M>(
        create: impl IntoModel<M> + ItemLike,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<M>
    where
        M: raw_crud::Create,
    {
        let mut item = create.as_item();
        let model = create.into_model(&item);

        item.owner_id = user.id;

        item.create(conn)?;
        model.create(conn)
    }

    pub fn update<M, U>(
        id: Uuid,
        update: U,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<M>
    where
        M: raw_crud::Update<U> + TypeMarker,
    {
        if Item::has_owner::<M>(id, user.id, conn) {
            M::update(id, update, conn)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn find<M>(id: Uuid, user: User, conn: &PgConnection) -> QueryResult<M>
    where
        M: raw_crud::Find + TypeMarker,
    {
        if Item::has_owner::<M>(id, user.id, conn) {
            M::find(id, conn)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn delete<M>(
        id: Uuid,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<()>
    where
        M: raw_crud::Delete + TypeMarker,
    {
        if Item::has_owner::<M>(id, user.id, conn) {
            M::delete(id, conn)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }
}

pub mod crud2http {
    use super::{intermidiate, IntoModel};
    use crate::{
        database::exec_on_pool,
        items::{ItemLike, TypeMarker},
        users::user::User,
        utils::responsable::Responsable,
        DbPool,
    };

    use actix_web::{Error, HttpResponse};
    use uuid::Uuid;

    pub async fn create<M, N>(
        create: N,
        user: User,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error>
    where
        N: 'static + Send + IntoModel<M> + ItemLike,
        M: 'static
            + Send
            + super::raw_crud::Create
            + serde::Serialize
            + TypeMarker,
    {
        exec_on_pool(pool, move |conn| intermidiate::create(create, user, conn))
            .await
            .into_response()
    }

    pub async fn find<M>(
        id: Uuid,
        user: User,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error>
    where
        M: 'static
            + Send
            + super::raw_crud::Find
            + serde::Serialize
            + TypeMarker,
    {
        exec_on_pool(pool, move |conn| intermidiate::find::<M>(id, user, conn))
            .await
            .into_response()
    }

    pub async fn update<M, U>(
        id: Uuid,
        update: U,
        user: User,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error>
    where
        U: 'static + Send,
        M: 'static
            + Send
            + super::raw_crud::Update<U>
            + serde::Serialize
            + TypeMarker,
    {
        exec_on_pool(pool, move |conn| {
            intermidiate::update::<M, U>(id, update, user, conn)
        })
        .await
        .into_response()
    }

    pub async fn delete<M>(
        id: Uuid,
        user: User,
        pool: &DbPool,
    ) -> Result<HttpResponse, Error>
    where
        M: 'static + Send + super::raw_crud::Delete + TypeMarker,
    {
        exec_on_pool(pool, move |conn| {
            intermidiate::delete::<M>(id, user, conn)
        })
        .await
        .into_response()
    }
}
