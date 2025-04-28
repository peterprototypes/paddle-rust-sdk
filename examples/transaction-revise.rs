use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .transaction_revise("txn_01jkfyfvnbgape1jf4fdhk8s9f")
        .customer_name("Revised Customer Name")
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
