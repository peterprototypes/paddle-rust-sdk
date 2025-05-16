use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .subscription_update_payment_method_transaction("sub_01jt0rbstf4v79k955pa7jhmjy")
        .await
        .unwrap();

    dbg!(response.data);
}
