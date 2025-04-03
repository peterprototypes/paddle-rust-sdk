use paddle_rust_sdk::{enums::TaxCategory, Paddle};

#[tokio::main]
pub async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let product = client
        .product_create("My Awesome Product", TaxCategory::Standard)
        .description("This is a test product")
        .custom_data([("internal_product_id".to_string(), "123".to_string())].into())
        .send()
        .await
        .unwrap();

    dbg!(product);
}
