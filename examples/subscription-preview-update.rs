use chrono::{Days, Utc};
use paddle_rust_sdk::{enums::ProrationBillingMode, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .subscription_preview_update("sub_01jt0rbstf4v79k955pa7jhmjy")
        .next_billed_at(Utc::now() + Days::new(10))
        .proration_billing_mode(ProrationBillingMode::ProratedImmediately)
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
