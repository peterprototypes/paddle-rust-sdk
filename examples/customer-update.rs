use chrono::{Months, Utc};
use paddle_rust_sdk::{enums::DiscountType, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let customer = client
        .customer_update("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .email("new@example.com")
        .send()
        .await
        .unwrap();

    dbg!(customer);
}
