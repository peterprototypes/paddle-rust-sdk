use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let list = client.discounts_list();
    let mut discounts = list.send();

    while let Some(res) = discounts.next().await.unwrap() {
        dbg!(res.data);
    }
}
