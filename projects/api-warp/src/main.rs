use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .and(warp::header("user-agent"))
        .map(|param_name: String, agent: String| {
            format!("Hello, {}, whose agent is {}!", param_name, agent)
        });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
