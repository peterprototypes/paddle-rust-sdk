use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let portal_session = client
        .create_portal_session("ctm_01jqztc78e1xfdgwhcgjzdrvgd")
        .send()
        .await
        .unwrap();

    dbg!(portal_session.data.urls.general.overview);
    dbg!(portal_session.data.urls.subscriptions);
}
