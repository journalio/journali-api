use crate::schema::tags;
use crate::users::user::User;
use diesel::pg::PgConnection;
use diesel::QueryResult;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Insertable)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub owner_id: Uuid,
}

impl Tag {
    fn from_partial(new_tag: NewTag, user: User) -> Self {
        let NewTag { name, color } = new_tag;
        Self { id: Uuid::new_v4(), name, color, owner_id: user.id }
    }
    
    fn create(
        new_tag: NewTag,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let tag = Self::from_partial(new_tag, user);

        diesel::insert_into(tags::table).values(&tag).get_result(conn)
    }

    fn update(id: Uuid, update_tag: UpdateTag, user: User, conn: &PgConnection) -> QueryResult<Self> {
        diesel::update(
            tags::table
                .filter(tags::columns::id.eq(id))
                .filter(tags::owner_id.eq(user.id)),
        )
        .set(update_tag)
        .get_result(conn)
    }

    fn delete(
        id: Uuid,
        user: User,
        connection: &PgConnection,
    ) -> QueryResult<()> {
        diesel::delete(
            tags::table
                .filter(tags::owner_id.eq(user.id))
                .filter(tags::id.eq(id))
        )
        .get_result::<Tag>(connection)
        .map(drop)
    }
}

#[derive(Deserialize)]
pub struct NewTag {
    name: String,
    color: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "tags"]
pub struct UpdateTag {
    name: String,
    color: String,
}

mod routes {
    use actix_web::{post, delete, patch, web, Error, HttpRequest, HttpResponse};

    use super::{NewTag, Tag, UpdateTag};
    use crate::database::exec_on_pool;
    use crate::utils::responsable::Responsable;
    use crate::DbPool;
    use uuid::Uuid;

    #[post("/tags")]
    pub async fn create_tag(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewTag>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        exec_on_pool(&pool, |conn| Tag::create(form.into_inner(), user, conn))
            .await
            .into_response()
    }

    #[patch("/tags/{id}")]
    pub async fn update_tag(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTag>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        exec_on_pool(&pool, move |conn| Tag::update(id.into_inner(), form.into_inner(), user, conn))
            .await
            .into_response()
    }
    
    #[delete("/tags/{id}")]
    pub async fn delete_tag(
        pool: web::Data<DbPool>,
        request: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = request.extensions().get().cloned().unwrap();
        exec_on_pool(&pool, |conn| Tag::delete(id.into_inner(), user, &conn))
            .await
            .into_response()
    }

    /*
        DIT DOE JE NIET: #[get(/tags/{item_id}/)] -> Vec<Tag>

        DIT IS BETER: #[get(/items/{id}/tags)] -> Vec<Tag>
    */
}
