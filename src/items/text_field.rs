use core::convert::AsRef;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::text_fields,
};

use super::{
    crud::{Create, Delete, Find, Update},
    item::OwnedItem,
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct TextField {
    pub id: Uuid,
    pub item_type: i16,
    pub text: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize)]
pub struct NewTextField {
    pub text: String,
    pub page_id: Uuid,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "text_fields"]
pub struct UpdateTextField {
    pub text: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

impl TypeMarker for TextField {
    const TYPE: ItemTypeNames = ItemTypeNames::TextField;
}

impl ItemLike for NewTextField {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        TextField::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.page_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Page as i16)
    }
}

impl Create for TextField {
    type Create = OwnedItem<NewTextField>;

    fn create(
        new_text_field: &Self::Create,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let text = new_text_field.as_ref().text.clone();

        let item = new_text_field.into_item();

        let text_field = Self {
            id: item.id,
            item_type: item.item_type,
            text,
            coord_x: new_text_field.as_ref().coord_x,
            coord_y: new_text_field.as_ref().coord_y,
        };

        item.create(conn)?;
        diesel::insert_into(text_fields::table)
            .values(&text_field)
            .get_result(conn)
    }
}

impl Find<(Uuid, crate::users::user::User)> for TextField {
    fn find(
        (id, user): (Uuid, crate::users::user::User),
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        use crate::schema;
        super::item::Item::belonging_to(&user)
            .inner_join(
                schema::text_fields::table
                    .on(schema::text_fields::id.eq(schema::items::id)),
            )
            .get_result::<(super::item::Item, TextField)>(conn)
            .map(|(_, text_field)| text_field)
    }
}

crate::impl_update! {
    for TextField {
        type Update = UpdateTextField;
        table = text_fields
    }
}
//impl Update for TextField {
//    type Update = OwnedItem<UpdateTextField>;
//
//    fn update(
//        id: Uuid,
//        form: &Self::Update,
//        conn: &PgConnection,
//    ) -> QueryResult<Self> {
//
//        use super::item::Item;
//        let update_text_field = form.as_ref();
//
//        if Item::has_owner::<Self>(id, form.user.id, conn) {
//            diesel::update(
//                text_fields::table
//                    .filter(text_fields::columns::id.eq(id))
//                    .filter(text_fields::item_type.eq(Self::TYPE as i16)),
//            )
//            .set(update_text_field)
//            .get_result(conn)
//        } else {
//            Err(diesel::result::Error::NotFound)
//        }
//    }
//}

impl Delete for TextField {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl TextField {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_text_field);
        cfg.service(routes::find_text_field);
        cfg.service(routes::update_text_field);
        cfg.service(routes::delete_text_field);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };
    use uuid::Uuid;

    use crate::{
        items::{crud::Crudder, item::OwnedItem},
        DbPool,
    };

    use super::{NewTextField, TextField, UpdateTextField};

    #[post("/text_fields")]
    pub async fn create_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewTextField>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        let owned_item = OwnedItem::new(user, form.into_inner());
        Crudder::<TextField>::create(owned_item, &pool).await
    }

    #[get("/text_fields/{id}")]
    pub async fn find_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        Crudder::<TextField>::find((id.into_inner(), user), &pool).await
    }

    #[patch("/text_fields/{id}")]
    pub async fn update_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTextField>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        let owned_item = OwnedItem::new(user, form.into_inner());

        Crudder::<TextField>::update(id.into_inner(), owned_item, &pool).await
    }

    #[delete("/text_fields/{id}")]
    pub async fn delete_text_field(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TextField>::delete(id.into_inner(), &pool).await
    }
}
