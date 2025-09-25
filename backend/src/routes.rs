pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::test]
async fn test_hello_world_handler() {
    let result = hello_world().await;
    assert_eq!(result, "Hello, World!");
}