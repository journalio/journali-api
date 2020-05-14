use actix_web::{Error, HttpResponse};
use serde::Serialize;

pub trait Responsable {
    fn into_response(self) -> Result<HttpResponse, Error>;
}

impl<T, E> Responsable for Result<T, E>
where
    T: Serialize,
{
    fn into_response(self) -> Result<HttpResponse, Error> {
        self.map(|item| HttpResponse::Ok().json(item))
            .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }
}
