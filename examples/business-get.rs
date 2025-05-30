use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let business = client
        .business_get(
            "ctm_01jqztc78e1xfdgwhcgjzdrvgd",
            "biz_01jr85bypq4d3w139m53zw2559",
        )
        .send()
        .await
        .unwrap();

    dbg!(business);
}
