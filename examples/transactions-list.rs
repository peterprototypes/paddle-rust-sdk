use chrono::Utc;
use paddle_rust_sdk::{enums::CollectionMode, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.transactions_list();
    let mut paginated = list
        .collection_mode(CollectionMode::Automatic)
        .billed_at_lt(Utc::now())
        // .customer_id(["ctm_01jk84f1s981kf2a4fqmv968ba"])
        .per_page(1)
        .send();

    while let Some(transactions) = paginated.next().await.unwrap() {
        dbg!(transactions);
    }
}
