use crate::api::beer::*;
use actix_producer::api::beer::*;

#[actix_web::test]
async fn should_verify() {
    use stubr::StubrVerify as _;

    actix_web::App::new()
        .app_data(sample_db())
        .service(create)
        .service(find_by_id)
        // reset application state
        .wrap(stubr::ActixVerifyLifecycle::<Database>(|db| {
            let mut db = db.write().unwrap();
            db.clear();
            for (i, beer) in sample() {
                db.insert(i, beer);
            }
        }))
        // required because this sample lives in a project with other stubs.
        // Otherwise just use '.verify()'
        .verify_except(|stub_name: &str| stub_name.starts_with("pet-"))
        .await;
}
