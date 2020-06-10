use super::tags_items::TagsItem;
use crate::schema::tags;
use crate::users::user::User;
use diesel::pg::PgConnection;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Insertable)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub owner_id: Uuid,
}

#[derive(Serialize)]
pub struct TagInfo {
    id: Uuid,
    name: String,
    color: String,
    owner_id: Uuid,
    items: Vec<(Uuid, i16)>,
}

impl Tag {
    fn from_partial(new_tag: NewTag, user: User) -> Self {
        let NewTag { name, color } = new_tag;
        Self { id: Uuid::new_v4(), name, color, owner_id: user.id }
    }

    fn find_all(
        user: User,
        connection: &PgConnection,
    ) -> QueryResult<Vec<TagInfo>> {
        let query = tags::table
            .filter(tags::columns::owner_id.eq(user.id))
            .into_boxed();

        query.load::<Tag>(connection).and_then(|tags| {
            tags.into_iter()
                .map(|tag| {
                    let tagsitems = TagsItem::find_all(tag.id, connection)?;

                    let tagsitems = tagsitems
                        .into_iter()
                        .map(|tagsitem| (tagsitem.item_id, tagsitem.item_type))
                        .collect::<Vec<_>>();

                    let Tag { id, name, color, owner_id } = tag;

                    Ok(TagInfo { id, name, color, owner_id, items: tagsitems })
                })
                .collect::<Result<Vec<_>, _>>()
        })
    }

    fn create(
        new_tag: NewTag,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let tag = Self::from_partial(new_tag, user);

        diesel::insert_into(tags::table).values(&tag).get_result(conn)
    }

    fn update(
        id: Uuid,
        update_tag: UpdateTag,
        user: User,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
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
                .filter(tags::id.eq(id)),
        )
        .get_result::<Tag>(connection)
        .map(drop)
    }
}

#[derive(Serialize, Deserialize)]
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

impl Tag {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::find_all);
        cfg.service(routes::create_tag);
        cfg.service(routes::update_tag);
        cfg.service(routes::delete_tag);
        cfg.service(routes::add_items_to_tag);
        cfg.service(routes::delete_items_from_tag);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };

    use super::{NewTag, Tag, UpdateTag};
    use crate::database::exec_on_pool;
    use crate::tags::tags_items::{TagsItem, TagsItemRequest};
    use crate::utils::responsable::Responsable;
    use crate::DbPool;
    use uuid::Uuid;

    #[patch("/tags/{id}/items")]
    pub async fn add_items_to_tag(
        pool: web::Data<DbPool>,
        request: HttpRequest,
        id: web::Path<Uuid>,
        items: web::Json<Vec<TagsItemRequest>>,
    ) -> Result<HttpResponse, Error> {
        let user = request.extensions().get().cloned().unwrap();

        exec_on_pool(&pool, |conn| {
            TagsItem::add_items(id.into_inner(), items.into_inner(), user, conn)
        })
        .await
        .into_response()
    }

    #[delete("/tags/{id}/items")]
    pub async fn delete_items_from_tag(
        pool: web::Data<DbPool>,
        request: HttpRequest,
        id: web::Path<Uuid>,
        items: web::Json<Vec<TagsItemRequest>>,
    ) -> Result<HttpResponse, Error> {
        let user = request.extensions().get().cloned().unwrap();

        exec_on_pool(&pool, |conn| {
            TagsItem::delete_items(
                id.into_inner(),
                items.into_inner(),
                user,
                conn,
            )
        })
        .await
        .into_response()
    }

    #[get("/tags")]
    pub async fn find_all(
        pool: web::Data<DbPool>,
        request: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        let user = request.extensions().get().cloned().unwrap();

        exec_on_pool(&pool, |conn| Tag::find_all(user, conn))
            .await
            .into_response()
    }

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

        exec_on_pool(&pool, move |conn| {
            Tag::update(id.into_inner(), form.into_inner(), user, conn)
        })
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

#[cfg(test)]
mod tests {
    use super::{NewTag, Tag};
    use crate::testing;

    #[actix_rt::test]
    async fn test_create_tag() -> Result<(), Box<dyn std::error::Error>> {
        testing::create::<_, NewTag>(
            Tag::routes,
            NewTag { name: "school".into(), color: "0xFFFFFF".into() },
            "/api/tags",
        )
        .await;

        Ok(())
    }
}
