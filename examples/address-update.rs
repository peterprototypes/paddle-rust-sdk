use paddle_rust_sdk::Paddle;

#[tokio::main]
async fn main() {
    let client = Paddle::new(std::env::var("PADDLE_API_KEY").unwrap(), Paddle::SANDBOX).unwrap();

    let address = client
        .address_update(
            "ctm_01jqztc78e1xfdgwhcgjzdrvgd",
            "add_01jr0ph4re1exn63cnyea7b65p",
        )
        .first_line("Updated first line")
        .send()
        .await
        .unwrap();

    dbg!(address);
}
