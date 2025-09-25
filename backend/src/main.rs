use backend::app;
use axum::serve;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    println!("Server running on http://{}", addr);
    serve(listener, app())
        .await
        .unwrap();
}
