use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let discount = client
        .discount_create("15", "Winter Holidays", DiscountType::Percentage)
        .enabled_for_checkout(true)
        .code("WIN2025")
        .usage_limit(2500)
        .expires_at(Utc::now() + Months::new(3))
        .custom_data([("utm_stuff".to_string(), "123".to_string())].into())
        .send()
        .await
        .unwrap();

    dbg!(discount);
}
