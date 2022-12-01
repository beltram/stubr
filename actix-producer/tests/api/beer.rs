use actix_web::{
    test::{call_service, init_service, TestRequest},
    web, App,
};
use asserhttp::*;

use actix_producer::api::beer::*;

mod find_by_id {
    use super::*;

    #[actix_web::test]
    async fn find_by_id_should_find_one() {
        let app = App::new()
            .app_data(sample_db())
            .service(find_by_id)
            .wrap(stubr::ActixRecord::default()); // 👈 record
        let beers = sample();
        let (id, to_find) = beers.get(0).unwrap();
        let req = TestRequest::get().uri(&format!("/beers/{}", id)).to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_ok()
            .expect_content_type_json()
            .expect_body_json(|b: Beer| assert_eq!(&b, to_find));
    }

    #[actix_web::test]
    async fn find_by_id_should_not_find_any() {
        let app = App::new()
            .app_data(sample_db())
            .service(find_by_id)
            .wrap(stubr::ActixRecord::default()); // 👈 record
        let req = TestRequest::get().uri("/beers/999").to_request();
        call_service(&init_service(app).await, req).await.expect_status_not_found();
    }
}

mod create {
    use super::*;

    #[actix_web::test]
    async fn create_should_create_one() {
        let beer = Beer {
            id: None,
            name: "Heineken".to_string(),
            price: 4,
        };
        let app = App::new()
            .app_data(empty_db())
            .service(create)
            .wrap(stubr::ActixRecord::default()); // 👈 record
        let req = TestRequest::post().uri("/beers").set_json(beer.clone()).to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_created()
            .expect_content_type_json()
            .expect_body_json(|b: Beer| {
                assert!(b.id.is_some());
                assert_eq!(b.name, beer.name);
                assert_eq!(b.price, beer.price);
            });
    }

    #[actix_web::test]
    async fn create_should_conflict_when_already_exists_by_name() {
        let (_id, beer) = sample().get(0).unwrap().clone();
        let app = App::new()
            .app_data(sample_db())
            .service(create)
            .wrap(stubr::ActixRecord::default()); // 👈 record
        let req = TestRequest::post().uri("/beers").set_json(beer.clone()).to_request();
        call_service(&init_service(app).await, req).await.expect_status_conflict();
    }
}

pub fn sample_db() -> web::Data<Database> {
    web::Data::new(std::sync::RwLock::new(sample().into()))
}

pub fn empty_db() -> web::Data<Database> {
    web::Data::new(std::sync::RwLock::new([].into()))
}

pub fn sample() -> [(u32, Beer); 2] {
    [
        (
            0,
            Beer {
                id: Some(0),
                name: "Leffe".to_string(),
                price: 5,
            },
        ),
        (
            1,
            Beer {
                id: Some(1),
                name: "1664".to_string(),
                price: 3,
            },
        ),
    ]
}
