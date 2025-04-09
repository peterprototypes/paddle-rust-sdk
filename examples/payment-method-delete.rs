use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    client
        .payment_method_delete(
            "ctm_01jk84f1s981kf2a4fqmv968ba",
            "paymtd_01jkfzx12e0awh6b3xd90m1h7h",
        )
        .await
        .unwrap();
}
