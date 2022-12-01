use actix_web::http::header::ContentType;
use actix_web::web;

// simple in-memory database
pub type Database = std::sync::RwLock<std::collections::HashMap<u32, Beer>>;

#[actix_web::post("/beers")]
pub async fn create(mut beer: web::Json<Beer>, db: web::Data<Database>) -> impl actix_web::Responder {
    let exists = db.read().unwrap().iter().any(|(_, b)| &beer.0 == b);
    if !exists {
        let next_id = db.read().unwrap().len() as u32;
        beer.id = Some(next_id);
        db.write().unwrap().insert(next_id, beer.clone());
        actix_web::HttpResponse::Created()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&beer).unwrap())
    } else {
        actix_web::HttpResponse::Conflict()
            .content_type(ContentType::json())
            .body(serde_json::json!({"message": "Beer already exists"}).to_string())
    }
}

#[actix_web::get("/beers/{id}")]
pub async fn find_by_id(path: web::Path<u32>, db: web::Data<Database>) -> impl actix_web::Responder {
    let id = path.into_inner();
    if let Some(beer) = db.read().unwrap().get(&id) {
        actix_web::HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(beer).unwrap())
    } else {
        actix_web::HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(serde_json::json!({"message": "Beer not found"}).to_string())
    }
}

#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize)]
pub struct Beer {
    pub id: Option<u32>,
    pub name: String,
    pub price: u32,
}

impl PartialEq for Beer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
