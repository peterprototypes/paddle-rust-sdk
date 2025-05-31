use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.customers_list();
    let mut paginated = list.per_page(1).send();
    let customers = paginated.all().await.unwrap();

    for customer in customers {
        dbg!(customer);
    }
}
