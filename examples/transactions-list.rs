use chrono::Utc;
use paddle_rust_sdk::{enums::CollectionMode, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let transactions = client
        .transactions_list()
        .collection_mode(CollectionMode::Automatic)
        .billed_at_lt(Utc::now())
        .customer_id(["ctm_01jk84f1s981kf2a4fqmv968ba"])
        .send()
        .await
        .unwrap();

    dbg!(transactions);
}
