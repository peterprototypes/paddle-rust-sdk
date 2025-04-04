use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let discount = client
        .discount_get("dsc_01jqzpbmnqpwta8q6agytc42hm")
        .send()
        .await
        .unwrap();

    dbg!(discount);
}
