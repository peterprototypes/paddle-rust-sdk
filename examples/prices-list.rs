use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let prices = client
        .prices_list()
        .order_by_asc("id")
        .per_page(20)
        .send()
        .await
        .unwrap();

    dbg!(prices);
}
