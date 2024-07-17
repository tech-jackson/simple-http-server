use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a route
    let hello = warp::path::end()
        .map(|| warp::reply::html("Hello, world!"));

    // Start the server on port 3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
