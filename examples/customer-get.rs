use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let customer = client
        .customer_get("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .send()
        .await
        .unwrap();

    dbg!(customer);
}
