//use warp::Reply;
use warp::Filter;

/*fn strVal(name: String)->Reply {
    println!("Got name : {}", name);
    format!("Hello, {}!", name)
}*/

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("str" / String).map(|name| {
        println!("Got name : {}", name);
        format!("Hello, {}!", name)
    });

    let port = 3030;
    println!("Serving the memory db at : {}", port);
    warp::serve(hello).run(([127, 0, 0, 1], port)).await;
}
