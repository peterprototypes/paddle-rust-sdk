use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let balances = client
        .customer_credit_balances("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .send()
        .await
        .unwrap();

    dbg!(balances);
}
