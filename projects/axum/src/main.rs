use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    // region:      --- Start server
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    // endregion:   --- End server

    // region:      --- Handler hello
    async fn handler_hello() -> impl IntoResponse {
        println!("--> {:<12} - handler_hello", "HANDLER");

        Html("Hello world!!!")
    }
    // endregion:      --- Handler hello
}
