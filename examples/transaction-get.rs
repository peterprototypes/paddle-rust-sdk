use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .transaction_get("txn_01jkfx8v9z4pee0p5bd35x95bp")
        .include(["address"])
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
