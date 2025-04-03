use paddle_rust_sdk::ids::ProductID;
use paddle_rust_sdk::Paddle;

#[tokio::main]
pub async fn main() {
    let client = Paddle::new("<YOUR_API_KEY>", Paddle::SANDBOX).unwrap();

    let products = client
        .products_list()
        .order_by_asc("id")
        .per_page(20)
        .status(paddle_rust_sdk::enums::Status::Active)
        .catalog_type(paddle_rust_sdk::enums::CatalogType::Standard)
        // .tax_category([paddle_rust_sdk::enums::TaxCategory::DigitalGoods])
        // .ids([ProductID::from("pro_01jk83xwchwd87hnywwqwhe1jt")])
        // .include(["prices"])
        .send()
        .await
        .unwrap();

    dbg!(products);
}
