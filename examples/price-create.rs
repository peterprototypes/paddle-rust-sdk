use paddle_rust_sdk::{
    enums::{CountryCodeSupported, CurrencyCode, Interval},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let price = client
        .price_create(
            "pro_01jqx9rdbdhs1zb1sj5v475fdz",
            "Internal price desc",
            999,
            CurrencyCode::USD,
        )
        .name("This is a test price")
        .billing_cycle(20, Interval::Day)
        .trial_period(10, Interval::Day)
        .add_unit_price_override([CountryCodeSupported::BG], 555, CurrencyCode::USD)
        .quantity(1..200)
        .custom_data([("grant_tokens".to_string(), "123".to_string())].into())
        .send()
        .await
        .unwrap();

    dbg!(price);
}
