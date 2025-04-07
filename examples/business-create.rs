use paddle_rust_sdk::{entities::Contact, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let address = client
        .business_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", "Company Inc.")
        .company_number("202835086")
        .tax_identifier("BG202835086")
        .contacts([Contact {
            name: "Primary Contact".into(),
            email: "test@example.com".into(),
        }])
        .send()
        .await
        .unwrap();

    dbg!(address);
}
