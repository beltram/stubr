use actix_web::{App, test::{call_service, init_service, TestRequest}, web};
use asserhttp::*;

use actix_consumer::{api::store, client::pet::PetClient, model::{pet::Pet, store::Store}};
use stubr::ActixRecord;

use crate::utils::*;

mod find_all {
    use super::*;

    #[actix_web::test]
    async fn find_all_should_find_some() {
        let app = App::new().app_data(fake_store_repository())
            .service(store::find_all)
            .wrap(ActixRecord::default());
        let req = TestRequest::get().uri("/stores").to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_partial_content()
            .expect_content_type_json()
            .expect_body_json(|b: Vec<Store>| assert_eq!(b.len(), fake_stores().len()));
    }

    #[actix_web::test]
    async fn find_all_should_find_none_when_empty() {
        let app = App::new().app_data(empty_store_repository()).service(store::find_all);
        let req = TestRequest::get().uri("/stores").to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_partial_content()
            .expect_content_type_json()
            .expect_body_json(|b: Vec<Store>| assert_eq!(b, vec![]));
    }
}

mod find_by_id {
    use super::*;

    #[actix_web::test]
    #[stubr::apps("actix-producer")]
    async fn find_by_id_should_find_one() {
        let app = App::new()
            .app_data(web::Data::new(PetClient::from(actix_producer.uri())))
            .app_data(fake_store_repository())
            .service(store::find_by_id)
            .wrap(ActixRecord::default());
        let stores = fake_stores();
        let to_find = stores.get(0).unwrap();
        let req = TestRequest::get().uri(&format!("/stores/{}", to_find.id.unwrap())).to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_ok()
            .expect_content_type_json()
            .expect_body_json(|p: Store| assert_eq!(&p, to_find));
    }

    #[actix_web::test]
    #[stubr::apps("actix-producer")]
    async fn find_by_id_should_not_find_any() {
        let app = App::new()
            .app_data(web::Data::new(PetClient::from(actix_producer.uri())))
            .app_data(fake_store_repository())
            .service(store::find_by_id)
            .wrap(ActixRecord::default());
        let req = TestRequest::get().uri("/stores/999").to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_not_found();
    }
}

mod create {
    use super::*;

    // TODO: figure out why fails on CI
    // #[actix_web::test]
    #[stubr::apps("actix-producer")]
    async fn create_should_create_one() {
        let pet = Pet { name: String::from("new"), ..Default::default() };
        let store = Store { name: String::from("new"), pets: vec![pet.clone()], ..Default::default() };
        let app = App::new()
            .app_data(web::Data::new(PetClient::from(actix_producer.uri())))
            .app_data(empty_store_repository())
            .service(store::create)
            .wrap(ActixRecord::default());
        let req = TestRequest::post().uri("/stores").set_json(store.clone()).to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_created()
            .expect_content_type_json()
            .expect_body_json(|b: Store| {
                assert!(b.id.is_some());
                assert_eq!(b.name, store.name);
                assert!(!b.pets.is_empty());
                let p = b.pets.get(0).unwrap();
                assert!(p.id.is_some());
                assert_eq!(p.name, pet.name);
            });
    }

    #[actix_web::test]
    #[stubr::apps("actix-producer")]
    async fn create_should_conflict_when_already_exists_by_name() {
        let mut store = fake_stores().get(0).unwrap().to_owned();
        store.id = None;
        let app = App::new()
            .app_data(web::Data::new(PetClient::from(actix_producer.uri())))
            .app_data(fake_store_repository())
            .service(store::create)
            .wrap(ActixRecord::default());
        let req = TestRequest::post().uri("/stores").set_json(store.clone()).to_request();
        call_service(&init_service(app).await, req).await
            .expect_status_conflict();
    }
}
