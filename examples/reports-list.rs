use paddle_rust_sdk::{enums::ReportStatus, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let mut list = client.reports_list();
    let mut paginated = list.status([ReportStatus::Ready]).send();

    while let Some(page) = paginated.next().await.unwrap() {
        dbg!(page.data);
    }
}
