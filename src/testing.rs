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
