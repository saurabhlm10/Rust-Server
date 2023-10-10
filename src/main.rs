use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a warp filter that responds to all requests with "Hello, Warp!"
    let hello = warp::path("hello").map(|| warp::reply::html("Hello, Rust!"));

    // Start the warp server on the specified address and port
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
