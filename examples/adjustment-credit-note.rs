use paddle_rust_sdk::{enums::Disposition, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .adjustment_credit_note("adj_01jvpb00pn0vywnjg7gqd9fe1w", Disposition::Inline)
        .await
        .unwrap();

    dbg!(res.data.url);
}
