use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let product = client
        .product_get("pro_01jqx9rdbdhs1zb1sj5v475fdz")
        .include(["prices"])
        .send()
        .await
        .unwrap();

    dbg!(product);
}
