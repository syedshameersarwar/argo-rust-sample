use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::any().map(|| "Hi!!! i am only for testing argo ambassador stack!");
    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}
