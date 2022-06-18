use parking_lot::RwLock;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

pub type Db = Arc<RwLock<HashMap<String, String>>>;

pub struct StrDb {
    db: Db,
}

pub impl StrDb {
    pub fn new() -> Self {
        StrDb {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn all(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> {
        self.list_filter()
            .or(self.get_filter())
            .or(self.upsert_filter())
    }

    //for GET /str
    fn list_filter(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> {
        warp::path!("str")
            .and(warp::get())
            .and_then(self.list_handler)
    }

    //the list function handler
    async fn list_handler(&self) -> Result<impl warp::Reply, Infallible> {
        // Just return a JSON array of todos, applying the limit and offset.
        let db = self.db.read();
        let vec: Vec<&str> = db.into_keys().collect();
        Ok(warp::reply::json(&vec))
    }

    //get a single value GET /str/:key
    fn get_filter(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("str" / String)
            .and(warp::get())
            .and_then(self.get_handler)
    }

    async fn get_handler(&self, key: String) -> Result<impl warp::Reply, Infallible> {
        let db = self.db.read();
        let val = db.get(&key);
        if let Some(v) = val {
            Ok(warp::reply::json(&v))
        } else {
            Ok(StatusCode::NOT_FOUND)
        }
    }

    /// POST /str/:key with JSON body
    fn upsert_filter(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("str" / String)
            .and(warp::post())
            .and(self.json_body())
            .and_then(self.upsert_handler)
    }

    async fn upsert_handler(
        &self,
        key: String,
        val: String,
    ) -> Result<impl warp::Reply, Infallible> {
        //log::debug!("upsert_handler: {}: {}", key, val);

        let mut db = self.db.write();

        db.insert(key, val);

        Ok(StatusCode::CREATED)
    }

    fn json_body(&self) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}
