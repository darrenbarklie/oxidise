use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /
    let hello_world = warp::path::end().map(|| "Hello, world!");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .and(warp::header("user-agent"))
        .map(|param_name: String, agent: String| {
            format!("Hello, {}, whose agent is {}!", param_name, agent)
        });

    let routes = warp::get().and(hello_world.or(hello));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
