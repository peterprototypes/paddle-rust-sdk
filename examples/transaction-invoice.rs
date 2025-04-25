use paddle_rust_sdk::{enums::Disposition, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .transaction_invoice("txn_01jkg0vcc7r0kpykrvxwham7t8", Disposition::Inline)
        .await
        .unwrap();

    dbg!(response.data.url);
}
