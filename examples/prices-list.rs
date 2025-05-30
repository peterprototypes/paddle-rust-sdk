use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut prices_list = client.prices_list();
    let mut prices = prices_list.order_by_asc("id").per_page(20).send();

    while let Some(res) = prices.next().await.unwrap() {
        dbg!(res.data);
    }
}
