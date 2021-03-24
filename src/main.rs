use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Crediantals {
    email: String,
    password: String
}



#[tokio::main]
async fn main() {

    println!("start");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello")
        .map(|| format!("Hello, tigidig!"));

    let login = warp::path!("login")
        .map(|| format!("Hello, tigidig! Login or kok!"));

    let routes = hello.or(login);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 80))
        .await;
}