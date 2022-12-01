use actix_web::{
    test::{call_service, init_service, TestRequest},
    App,
};
use asserhttp::*;

use actix_producer::{api::pet, model::pet::Pet};
use stubr::ActixRecord;

use crate::utils::*;

mod find_all {
    use super::*;

    #[actix_web::test]
    async fn find_all_should_find_some() {
        let app = App::new()
            .app_data(fake_pet_repository())
            .service(pet::find_all)
            .wrap(ActixRecord::default());
        let req = TestRequest::get().uri("/pets").to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_partial_content()
            .expect_content_type_json()
            .expect_body_json(|b: Vec<Pet>| assert_eq!(b.len(), fake_pets().len()));
    }

    #[actix_web::test]
    async fn find_all_should_find_none_when_empty() {
        let app = App::new().app_data(empty_pet_repository()).service(pet::find_all);
        let req = TestRequest::get().uri("/pets").to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_partial_content()
            .expect_content_type_json()
            .expect_body_json(|b: Vec<Pet>| assert_eq!(b, vec![]));
    }
}

mod find_by_id {
    use super::*;

    #[actix_web::test]
    async fn find_by_id_should_find_one() {
        let app = App::new()
            .app_data(fake_pet_repository())
            .service(pet::find_by_id)
            .wrap(ActixRecord::default());
        let pets = fake_pets();
        let to_find = pets.get(0).unwrap();
        let req = TestRequest::get().uri(&format!("/pets/{}", to_find.id.unwrap())).to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_ok()
            .expect_content_type_json()
            .expect_body_json(|p: Pet| assert_eq!(&p, to_find));
    }

    #[actix_web::test]
    async fn find_by_id_should_not_find_any() {
        let app = App::new()
            .app_data(fake_pet_repository())
            .service(pet::find_by_id)
            .wrap(ActixRecord::default());
        let req = TestRequest::get().uri("/pets/999").to_request();
        call_service(&init_service(app).await, req).await.expect_status_not_found();
    }
}

mod create {
    use super::*;

    #[actix_web::test]
    async fn create_should_create_one() {
        let pet = Pet {
            name: String::from("new"),
            ..Default::default()
        };
        let app = App::new()
            .app_data(empty_pet_repository())
            .service(pet::create)
            .wrap(ActixRecord::default());
        let req = TestRequest::post().uri("/pets").set_json(pet.clone()).to_request();
        call_service(&init_service(app).await, req)
            .await
            .expect_status_created()
            .expect_content_type_json()
            .expect_body_json(|b: Pet| {
                assert!(b.id.is_some());
                assert_eq!(b.name, pet.name);
            });
    }

    #[actix_web::test]
    async fn create_should_conflict_when_already_exists_by_name() {
        let pet = fake_pets().get(0).unwrap().to_owned();
        let app = App::new()
            .app_data(fake_pet_repository())
            .service(pet::create)
            .wrap(ActixRecord::default());
        let req = TestRequest::post().uri("/pets").set_json(pet.clone()).to_request();
        call_service(&init_service(app).await, req).await.expect_status_conflict();
    }
}
