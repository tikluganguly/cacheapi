use crate::db::strdb::StrDb;

mod db {
    pub mod strdb;
}

#[tokio::main]
async fn main() {
    let db = StrDb::new();

    let port = 3030;
    println!("Serving the memory db at : {}", port);
    warp::serve(db.all()).run(([127, 0, 0, 1], port)).await;
}
