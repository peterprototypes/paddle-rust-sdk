use paddle_rust_sdk::{
    enums::{CollectionMode, SubscriptionStatus},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .subscriptions_list()
        .status([SubscriptionStatus::Trialing])
        .per_page(25)
        .collection_mode(CollectionMode::Automatic)
        .send()
        .await
        .unwrap();

    dbg!(res.data);
}
