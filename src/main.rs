use warp::Filter;

#[tokio::main]
async fn main() {


    println!("start");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello")
        .map(|| format!("Hello, tigidig!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 80))
        .await;
}