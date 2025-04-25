use paddle_rust_sdk::{
    entities::{TransactionItemNonCatalogPrice, TransactionSubscriptionProductCreate},
    enums::{CurrencyCode, TaxCategory},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let transaction = client
        .transaction_preview()
        .append_catalog_item("pri_01jqxvdyjkp961jzv4me7ezg4d", 1)
        .append_non_catalog_item(
            TransactionItemNonCatalogPrice::new(
                "Custom non catalog price for non catalog product",
                1000,
                CurrencyCode::USD,
            )
            .name("This is a test")
            .product(TransactionSubscriptionProductCreate {
                name: "Test".into(),
                description: Some("Test".into()),
                tax_category: TaxCategory::Standard,
                image_url: None,
                custom_data: None,
            }),
            1,
        )
        .append_non_catalog_item(
            TransactionItemNonCatalogPrice::new(
                "Custom non catalog price for catalog product",
                1000,
                CurrencyCode::USD,
            )
            .name("This is a test")
            .product_id("pro_01jqx9rdbdhs1zb1sj5v475fdz"),
            1,
        )
        .send()
        .await
        .unwrap();

    dbg!(transaction);
}
