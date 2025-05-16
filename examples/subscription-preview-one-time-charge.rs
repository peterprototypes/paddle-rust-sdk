use paddle_rust_sdk::{enums::EffectiveFrom, transactions::TransactionItem, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let response = client
        .subscription_preview_one_time_charge("sub_01jt0rbstf4v79k955pa7jhmjy")
        .items([TransactionItem::CatalogItem {
            price_id: "pri_01jk83yyzeb91c32t3tktt8697".into(),
            quantity: 1,
        }])
        .effective_from(EffectiveFrom::Immediately)
        .send()
        .await
        .unwrap();

    dbg!(response.data);
}
