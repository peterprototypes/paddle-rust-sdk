use paddle_rust_sdk::{
    enums::{AdjustmentAction, AdjustmentType},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .adjustment_create(
            "txn_01jkfx8v9z4pee0p5bd35x95bp",
            AdjustmentAction::Refund,
            "Testing",
        )
        .r#type(AdjustmentType::Full)
        .send()
        .await
        .unwrap();

    dbg!(res.data);
}
