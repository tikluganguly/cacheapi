use crate::db::strdb::StrDb;

use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StrId {
    id: String,
}

#[get("/str/{id}")]
pub async fn get_task(str_id: Path<StrId>) -> Json<String> {
    let db = StrDb::new();
    Json(db.get(str_id.into_inner().id).await)
}
