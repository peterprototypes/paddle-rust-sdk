use paddle_rust_sdk::{
    enums::{CollectionMode, SubscriptionStatus},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.subscriptions_list();
    let mut paginated = list
        // .status([SubscriptionStatus::Trialing])
        .per_page(25)
        .collection_mode(CollectionMode::Automatic)
        .send();

    while let Some(page) = paginated.next().await.unwrap() {
        dbg!(page.data);
    }
}
