use core::convert::AsRef;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::pages,
};

use super::{
    crud::{Create, Delete, Find, Update},
    item::OwnedItem,
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct Page {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
}

#[derive(Clone, Deserialize)]
pub struct NewPage {
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "pages"]
pub struct UpdatePage {
    pub title: String,
}

impl TypeMarker for Page {
    const TYPE: ItemTypeNames = ItemTypeNames::Page;
}

impl ItemLike for NewPage {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        Page::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn parent_type(&self) -> Option<i16> {
        None
    }
}

impl Create for Page {
    type Create = OwnedItem<NewPage>;

    fn create(
        new_page: &Self::Create,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let title = new_page.as_ref().title.clone();
        let item = new_page.into_item();

        let page = Self { id: item.id, item_type: item.item_type, title };

        item.create(conn)?;
        diesel::insert_into(pages::table).values(&page).get_result(conn)
    }
}

use diesel::BelongingToDsl;

impl Find<(Uuid, crate::users::user::User)> for Page {
    fn find(
        (id, user): (Uuid, crate::users::user::User),
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        use crate::schema;
        super::item::Item::belonging_to(&user)
            .inner_join(
                schema::pages::table
                    .on(schema::pages::id.eq(schema::items::id)),
            )
            .get_result::<(super::item::Item, Page)>(conn)
            .map(|(_, page)| page)
    }
}

crate::impl_update! {
    for Page {
        type Update = UpdatePage;
        table = pages
    }
}
//impl Update for Page {
//    type Update = OwnedItem<UpdatePage>;
//
//    fn update(
//        id: Uuid,
//        form: &Self::Update,
//        conn: &PgConnection,
//    ) -> QueryResult<Self> {
//        use super::item::Item;
//        let update_page = form.as_ref();
//        
//        if Item::has_owner::<Self>(id, form.user.id, conn) {
//            diesel::update(
//                pages::table
//                    .filter(pages::columns::id.eq(id))
//                    .filter(pages::item_type.eq(Self::TYPE as i16)),
//            )
//            .set(update_page)
//            .get_result(conn)     
//        } else {
//            Err(diesel::result::Error::NotFound)
//        } 
//    }
//}

impl Delete for Page {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl Page {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_page);
        cfg.service(routes::find_page);
        cfg.service(routes::update_page);
        cfg.service(routes::delete_page);
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

    use super::{NewPage, Page, UpdatePage};

    #[post("/pages")]
    pub async fn create_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewPage>,
    ) -> Result<HttpResponse, Error> {
        let user = req
            .extensions()
            .get()
            .cloned()
            .unwrap();
        
        let owned_item = OwnedItem::new(user, form.into_inner());
        Crudder::<Page>::create(owned_item, &pool).await
    }

    #[get("/pages/{id}")]
    pub async fn find_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req
            .extensions()
            .get()
            .cloned()
            .unwrap();
        
        Crudder::<Page>::find((id.into_inner(), user), &pool).await
    }

    #[patch("/pages/{id}")]
    pub async fn update_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdatePage>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        let owned_item = OwnedItem::new(user, form.into_inner());

        Crudder::<Page>::update(id.into_inner(), owned_item, &pool).await
    }

    #[delete("/pages/{id}")]
    pub async fn delete_page(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Page>::delete(id.into_inner(), &pool).await
    }
}
