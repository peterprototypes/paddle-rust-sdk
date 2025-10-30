use paddle_rust_sdk::{enums::SubscriptionInclude, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .subscription_get("sub_01jt0rbstf4v79k955pa7jhmjy")
        .include([
            SubscriptionInclude::NextTransaction,
            SubscriptionInclude::RecurringTransactionDetails,
        ])
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
