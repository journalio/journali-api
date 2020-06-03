use crate::schema::tags;
use crate::users::user::User;
use diesel::pg::PgConnection;
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
}

#[derive(Deserialize)]
pub struct NewTag {
    name: String,
    color: String,
}

mod routes {
    use actix_web::{post, web, Error, HttpRequest, HttpResponse};

    use super::{NewTag, Tag};
    use crate::database::exec_on_pool;
    use crate::utils::responsable::Responsable;
    use crate::DbPool;
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
}
