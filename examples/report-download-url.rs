use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .report_download_url("rep_01jvvgpvqgxawryb6gw4xv4pzx")
        .await
        .unwrap();

    dbg!(res.data);
}
