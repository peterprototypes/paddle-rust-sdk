use paddle_rust_sdk::ids::ProductID;
use paddle_rust_sdk::Paddle;

#[tokio::main]
pub async fn main() {
    let client = Paddle::new(
        "b4a594c6a0acdad7b84d817d9c0ef4f5117032dd87e27bb099",
        Paddle::SANDBOX,
    )
    .unwrap();

    let products = client
        .products_list()
        // .ids([ProductID::from("pro_01jk83xwchwd87hnywwqwhe1jt")])
        // .include(["prices"])
        .order_by_asc("id")
        .per_page(20)
        .status(paddle_rust_sdk::enums::Status::Active)
        // .tax_category([paddle_rust_sdk::enums::TaxCategory::DigitalGoods])
        .catalog_type(paddle_rust_sdk::enums::CatalogType::Standard)
        .send()
        .await
        .unwrap();

    dbg!(products);
}
