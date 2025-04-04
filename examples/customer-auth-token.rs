use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let token = client
        .generate_auth_token("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .await
        .unwrap();

    dbg!(token);
}
