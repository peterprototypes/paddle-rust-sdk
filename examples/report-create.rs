use paddle_rust_sdk::{
    entities::ReportFilterValue,
    enums::{TransactionsReportFilterName, TransactionsReportType},
    Paddle,
};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let res = client
        .report_create(TransactionsReportType::Transactions)
        .append_filter(
            TransactionsReportFilterName::Status,
            None,
            ReportFilterValue::Array(vec!["past_due".to_string()]),
        )
        .send()
        .await
        .unwrap();

    dbg!(res.data);
}
