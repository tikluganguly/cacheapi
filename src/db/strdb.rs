use parking_lot::RwLock;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

pub type Db = Arc<RwLock<HashMap<String, String>>>;

#[derive(Clone)]
pub struct StrDb {
    db: Db,
}

impl StrDb {
    pub fn new() -> Self {
        StrDb {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    //the list function handler
    async fn list(&self) -> Vec<String> {
        // Just return a JSON array of todos, applying the limit and offset.
        let db = self.db.read();
        let vec: Vec<String> = db.to_owned().into_keys().collect();
        vec
    }

    async fn get(&self, key: String) -> String {
        let db = self.db.read();
        let val = db.get(&key);
        match val {
            Some(z) => z.to_string(),
            None => "".to_string(),
        }
    }
    async fn upsert(&self, key: String, val: String) {
        self.db.write().insert(key, val);
    }
}
