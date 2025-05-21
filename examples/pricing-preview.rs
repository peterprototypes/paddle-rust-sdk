use paddle_rust_sdk::{entities::PricePreviewItem, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .pricing_preview([PricePreviewItem {
            price_id: "pri_01jqxvdyjkp961jzv4me7ezg4d".into(),
            quantity: 1,
        }])
        .send()
        .await
        .unwrap();

    dbg!(res.data);
}
