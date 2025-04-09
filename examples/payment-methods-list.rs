use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let payment_methods = client
        .payment_methods_list("ctm_01jk84f1s981kf2a4fqmv968ba")
        .send()
        .await
        .unwrap();

    dbg!(payment_methods);
}
