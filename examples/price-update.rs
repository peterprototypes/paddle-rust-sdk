use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let price = client
        .price_update("pri_01jqxvdyjkp961jzv4me7ezg4d")
        .name("Updated name")
        .send()
        .await
        .unwrap();

    dbg!(price);
}
