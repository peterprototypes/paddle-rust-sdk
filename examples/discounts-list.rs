use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let discounts = client.discounts_list().send().await.unwrap();

    dbg!(discounts);
}
