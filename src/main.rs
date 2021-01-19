use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::any().map(|| "Hiiiiii!!!!!! I am testing k8s argo ambassador stack on linode with succcess finally!!!!!!!");
    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}
