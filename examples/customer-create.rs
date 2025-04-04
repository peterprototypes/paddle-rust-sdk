use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let customer = client
        .customer_create("test@exmaple.com")
        .name("John Doe")
        .send()
        .await
        .unwrap();

    dbg!(customer);
}
