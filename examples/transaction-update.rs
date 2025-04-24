use paddle_rust_sdk::{enums::TransactionStatus, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .transaction_update("txn_01jkfx8v9z4pee0p5bd35x95bp")
        .include(["address"])
        .status(TransactionStatus::Billed)
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
