use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut products_list = client.products_list();
    let mut products = products_list.order_by_asc("id").per_page(20).send();

    while let Some(products) = products.next().await.unwrap() {
        dbg!(products);
    }
}
