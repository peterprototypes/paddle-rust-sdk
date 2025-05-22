use paddle_rust_sdk::{enums::ReportStatus, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .reports_list()
        .status([ReportStatus::Ready])
        .send()
        .await
        .unwrap();

    dbg!(res.data);
}
