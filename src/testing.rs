#[macro_export]
macro_rules! test {
    (setup $setup:block test = |$app:ident| $($test:tt)*) => {
        let app = actix_web::test::init_service(
            actix_web::App::new()
                .data(crate::database::create_pool())
                .configure($setup)
        ).await;
        
        let mut $app = app;
        $($test)*
    }
}

use crate::{create_pool, utils::{jwt::Token, validator}, users::user::User};

use actix_web::{test, web::{
    self,
    ServiceConfig,
}, http::{StatusCode, header}};
use actix_web_httpauth::middleware::HttpAuthentication;

pub async fn create<Configure, Create, Model, Assert>(configure: Configure, create: Create, uri: &str, assert: Assert)
where
    Create: serde::Serialize,
    Model: serde::Serialize + for<'d> serde::Deserialize<'d>, 
    Assert: FnOnce(Model),
    Configure: FnOnce(&mut ServiceConfig)
{
    let auth = HttpAuthentication::bearer(validator);
        
        let mut app = actix_web::test::init_service(
            actix_web::App::new()
                .data(create_pool())
                .service(
                    web::scope("/api")
                        .configure(User::routes).service(
                            web::scope("")
                            .wrap(auth)
                            .configure(configure)
                        )
                )
        ).await;
        
    let user = r#"{"username":"tester","password": "simple"}"#.as_bytes();
    
    // REGISTER USER
    {
        let request = test::TestRequest::post().uri("/api/register")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(user)
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    // LOGIN USER
    let jwt = {
        let request = test::TestRequest::post().uri("/api/login")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(user)
            .to_request();

        let token: Token = test::read_response_json(&mut app, request).await;
        token.token
    };

    // CREATE REQUEST
    {
        let request = test::TestRequest::post().uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("{}", jwt))
            .set_json(&create)
            .to_request();

        let response: Model = test::read_response_json(&mut app, request).await;

        assert(response);
    }
}

