use warp::Filter;

#[tokio::main]
async fn main() {


    println!("start");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hi" / String)
        .map(|name| format!("Hello, tigidig: {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}