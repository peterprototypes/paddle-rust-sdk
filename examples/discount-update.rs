use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let discount = client
        .discount_update("dsc_01jqzpbmnqpwta8q6agytc42hm")
        .amount("18")
        .send()
        .await
        .unwrap();

    dbg!(discount);
}
