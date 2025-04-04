use paddle_rust_sdk::{enums::CountryCodeSupported, Paddle};

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let address = client
        .address_create("ctm_01jqztc78e1xfdgwhcgjzdrvgd", CountryCodeSupported::US)
        .first_line("Address first line")
        .second_line("Address second line")
        .city("Atlanta")
        .postal_code("30033")
        .region("GA")
        .send()
        .await
        .unwrap();

    dbg!(address);
}
