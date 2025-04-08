use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let businesses = client
        .businesses_list("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .send()
        .await
        .unwrap();

    dbg!(businesses);
}
